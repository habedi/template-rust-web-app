use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::domain::entities::items::{DESCRIPTION_MAX_LEN, ItemDraft, NAME_MAX_LEN};
use crate::domain::error::Error;
use crate::infrastructure::web::State as AppState;
use crate::infrastructure::web::middleware::ValidatedJson;

#[derive(Debug, Deserialize, Validate)]
struct ItemBody {
    #[validate(length(min = 1, max = NAME_MAX_LEN))]
    name: String,
    #[validate(length(max = DESCRIPTION_MAX_LEN))]
    description: Option<String>,
}

impl ItemBody {
    fn into_draft(self) -> Result<ItemDraft, Error> {
        ItemDraft::new(&self.name, self.description.as_deref())
    }
}

async fn list_items(State(state): State<AppState>) -> Result<impl IntoResponse, Error> {
    let items = state.items.list_items().await?;
    Ok((StatusCode::OK, Json(items)))
}

async fn get_item(
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let item = state.items.get_item(item_id).await?;
    Ok((StatusCode::OK, Json(item)))
}

async fn post_item(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<ItemBody>,
) -> Result<impl IntoResponse, Error> {
    let item = state.items.create_item(payload.into_draft()?).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

async fn put_item(
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<ItemBody>,
) -> Result<impl IntoResponse, Error> {
    let item = state
        .items
        .replace_item(item_id, payload.into_draft()?)
        .await?;
    Ok((StatusCode::OK, Json(item)))
}

async fn delete_item(
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    state.items.delete_item(item_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_items).post(post_item))
        .route(
            "/{item_id}",
            get(get_item).put(put_item).delete(delete_item),
        )
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use mockall::predicate;

    use super::*;
    use crate::application::use_cases::items::MockItemsUseCase;
    use crate::domain::entities::items::Item;
    use crate::infrastructure::web::{get_mock_state, routes::extract_body_response};

    fn sample_item(name: &str) -> Item {
        ItemDraft::new(name, None)
            .unwrap()
            .into_new_item(Utc::now())
    }

    #[tokio::test]
    async fn list_items_returns_the_items() {
        let items = vec![sample_item("widget")];
        let expected = items.clone();

        let mut use_case = MockItemsUseCase::new();
        use_case.expect_list_items().return_once(move || Ok(items));

        let response = super::list_items(axum::extract::State(get_mock_state(use_case)))
            .await
            .unwrap()
            .into_response();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            extract_body_response::<Vec<Item>>(response.into_body())
                .await
                .unwrap(),
            expected
        );
    }

    #[tokio::test]
    async fn get_item_returns_the_item() {
        let item = sample_item("widget");
        let id = item.id;
        let expected = item.clone();

        let mut use_case = MockItemsUseCase::new();
        use_case
            .expect_get_item()
            .with(predicate::eq(id))
            .return_once(move |_| Ok(item));

        let response = super::get_item(
            axum::extract::State(get_mock_state(use_case)),
            axum::extract::Path(id),
        )
        .await
        .unwrap()
        .into_response();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            extract_body_response::<Item>(response.into_body())
                .await
                .unwrap(),
            expected
        );
    }

    #[tokio::test]
    async fn get_item_reports_a_missing_item() {
        let mut use_case = MockItemsUseCase::new();
        use_case
            .expect_get_item()
            .return_once(|_| Err(Error::not_found()));

        let response = super::get_item(
            axum::extract::State(get_mock_state(use_case)),
            axum::extract::Path(Uuid::new_v4()),
        )
        .await
        .err()
        .unwrap()
        .into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn post_item_returns_201_with_the_created_item() {
        let item = sample_item("widget");
        let expected = item.clone();

        let mut use_case = MockItemsUseCase::new();
        use_case
            .expect_create_item()
            .withf(|draft: &ItemDraft| draft.name() == "widget")
            .return_once(move |_| Ok(item));

        let response = super::post_item(
            axum::extract::State(get_mock_state(use_case)),
            ValidatedJson(ItemBody {
                name: "widget".to_string(),
                description: None,
            }),
        )
        .await
        .unwrap()
        .into_response();

        assert_eq!(response.status(), StatusCode::CREATED);
        assert_eq!(
            extract_body_response::<Item>(response.into_body())
                .await
                .unwrap(),
            expected
        );
    }

    #[tokio::test]
    async fn post_item_rejects_a_blank_name() {
        let mut use_case = MockItemsUseCase::new();
        use_case.expect_create_item().never();

        let response = super::post_item(
            axum::extract::State(get_mock_state(use_case)),
            ValidatedJson(ItemBody {
                name: "   ".to_string(),
                description: None,
            }),
        )
        .await
        .err()
        .unwrap()
        .into_response();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn put_item_returns_the_updated_item() {
        let item = sample_item("gadget");
        let id = item.id;
        let expected = item.clone();

        let mut use_case = MockItemsUseCase::new();
        use_case
            .expect_replace_item()
            .withf(move |arg_id: &Uuid, draft: &ItemDraft| {
                *arg_id == id && draft.name() == "gadget"
            })
            .return_once(move |_, _| Ok(item));

        let response = super::put_item(
            axum::extract::State(get_mock_state(use_case)),
            axum::extract::Path(id),
            ValidatedJson(ItemBody {
                name: "gadget".to_string(),
                description: None,
            }),
        )
        .await
        .unwrap()
        .into_response();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            extract_body_response::<Item>(response.into_body())
                .await
                .unwrap(),
            expected
        );
    }

    #[tokio::test]
    async fn delete_item_returns_204() {
        let item = sample_item("widget");
        let id = item.id;

        let mut use_case = MockItemsUseCase::new();
        use_case
            .expect_delete_item()
            .with(predicate::eq(id))
            .return_once(move |_| Ok(item));

        let response = super::delete_item(
            axum::extract::State(get_mock_state(use_case)),
            axum::extract::Path(id),
        )
        .await
        .unwrap()
        .into_response();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
