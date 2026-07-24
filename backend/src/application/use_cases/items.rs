use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use crate::application::services::items::ItemService;
use crate::domain::entities::items::{Item, ItemDraft};
use crate::domain::error::Result;

#[async_trait]
pub trait ItemsUseCaseTrait: Send + Sync {
    async fn list_items(&self) -> Result<Vec<Item>>;
    async fn get_item(&self, id: Uuid) -> Result<Item>;
    async fn create_item(&self, draft: ItemDraft) -> Result<Item>;
    async fn replace_item(&self, id: Uuid, draft: ItemDraft) -> Result<Item>;
    async fn delete_item(&self, id: Uuid) -> Result<Item>;
}

pub struct ItemsUseCase {
    item_service: Box<dyn ItemService>,
}

impl ItemsUseCase {
    pub fn new(item_service: Box<dyn ItemService>) -> Self {
        Self { item_service }
    }
}

#[async_trait]
impl ItemsUseCaseTrait for ItemsUseCase {
    async fn list_items(&self) -> Result<Vec<Item>> {
        self.item_service.find_all().await
    }

    async fn get_item(&self, id: Uuid) -> Result<Item> {
        self.item_service.find_by_id(id).await
    }

    async fn create_item(&self, draft: ItemDraft) -> Result<Item> {
        let item = draft.into_new_item(Utc::now());
        self.item_service.insert(item).await
    }

    async fn replace_item(&self, id: Uuid, draft: ItemDraft) -> Result<Item> {
        // Load first so a missing item reports as not found rather than as an
        // update that silently affected no rows.
        let existing = self.item_service.find_by_id(id).await?;
        let updated = draft.apply_to(existing, Utc::now());
        self.item_service.update(updated).await
    }

    async fn delete_item(&self, id: Uuid) -> Result<Item> {
        self.item_service.delete(id).await
    }
}

#[cfg(test)]
mockall::mock! {
    pub ItemsUseCase {}
    #[async_trait]
    impl ItemsUseCaseTrait for ItemsUseCase {
        async fn list_items(&self) -> Result<Vec<Item>>;
        async fn get_item(&self, id: Uuid) -> Result<Item>;
        async fn create_item(&self, draft: ItemDraft) -> Result<Item>;
        async fn replace_item(&self, id: Uuid, draft: ItemDraft) -> Result<Item>;
        async fn delete_item(&self, id: Uuid) -> Result<Item>;
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;

    use super::*;
    use crate::application::services::items::MockItemService;
    use crate::domain::error::{Error, RepositoryErrorType};

    fn sample_item(name: &str) -> Item {
        ItemDraft::new(name, None)
            .unwrap()
            .into_new_item(Utc::now())
    }

    #[tokio::test]
    async fn list_items_returns_every_item() {
        let items = vec![sample_item("widget")];
        let expected = items.clone();

        let mut item_service = MockItemService::new();
        item_service
            .expect_find_all()
            .return_once(move || Ok(items));

        let use_case = ItemsUseCase::new(Box::new(item_service));

        assert_eq!(use_case.list_items().await.unwrap(), expected);
    }

    #[tokio::test]
    async fn get_item_returns_the_matching_item() {
        let item = sample_item("widget");
        let id = item.id;
        let expected = item.clone();

        let mut item_service = MockItemService::new();
        item_service
            .expect_find_by_id()
            .with(predicate::eq(id))
            .return_once(move |_| Ok(item));

        let use_case = ItemsUseCase::new(Box::new(item_service));

        assert_eq!(use_case.get_item(id).await.unwrap(), expected);
    }

    #[tokio::test]
    async fn get_item_propagates_not_found() {
        let mut item_service = MockItemService::new();
        item_service
            .expect_find_by_id()
            .return_once(|_| Err(Error::not_found()));

        let use_case = ItemsUseCase::new(Box::new(item_service));

        assert!(matches!(
            use_case.get_item(Uuid::new_v4()).await.unwrap_err(),
            Error::Repository(RepositoryErrorType::NotFound)
        ));
    }

    #[tokio::test]
    async fn create_item_stamps_an_id_and_matching_timestamps() {
        let mut item_service = MockItemService::new();
        item_service
            .expect_insert()
            .withf(|item: &Item| {
                item.name == "widget"
                    && item.description.as_deref() == Some("a thing")
                    && !item.id.is_nil()
                    && item.created_at == item.updated_at
            })
            .return_once(Ok);

        let use_case = ItemsUseCase::new(Box::new(item_service));
        let draft = ItemDraft::new("widget", Some("a thing")).unwrap();

        let created = use_case.create_item(draft).await.unwrap();

        assert_eq!(created.name, "widget");
        assert_eq!(created.created_at, created.updated_at);
    }

    #[tokio::test]
    async fn replace_item_keeps_creation_time_and_moves_updated_at() {
        let existing = sample_item("old");
        let id = existing.id;
        let created_at = existing.created_at;

        let mut item_service = MockItemService::new();
        item_service
            .expect_find_by_id()
            .with(predicate::eq(id))
            .return_once(move |_| Ok(existing));
        item_service
            .expect_update()
            .withf(move |item: &Item| {
                item.id == id
                    && item.name == "new"
                    && item.created_at == created_at
                    && item.updated_at >= created_at
            })
            .return_once(Ok);

        let use_case = ItemsUseCase::new(Box::new(item_service));
        let draft = ItemDraft::new("new", None).unwrap();

        let updated = use_case.replace_item(id, draft).await.unwrap();

        assert_eq!(updated.id, id);
        assert_eq!(updated.created_at, created_at);
    }

    #[tokio::test]
    async fn replace_item_does_not_update_a_missing_item() {
        let mut item_service = MockItemService::new();
        item_service
            .expect_find_by_id()
            .return_once(|_| Err(Error::not_found()));
        item_service.expect_update().never();

        let use_case = ItemsUseCase::new(Box::new(item_service));
        let draft = ItemDraft::new("new", None).unwrap();

        assert!(matches!(
            use_case
                .replace_item(Uuid::new_v4(), draft)
                .await
                .unwrap_err(),
            Error::Repository(RepositoryErrorType::NotFound)
        ));
    }

    #[tokio::test]
    async fn delete_item_returns_the_removed_item() {
        let item = sample_item("widget");
        let id = item.id;
        let expected = item.clone();

        let mut item_service = MockItemService::new();
        item_service
            .expect_delete()
            .with(predicate::eq(id))
            .return_once(move |_| Ok(item));

        let use_case = ItemsUseCase::new(Box::new(item_service));

        assert_eq!(use_case.delete_item(id).await.unwrap(), expected);
    }
}
