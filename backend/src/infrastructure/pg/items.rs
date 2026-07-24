use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use uuid::Uuid;

use crate::application::services::items::ItemService;
use crate::domain::entities::items::Item;
use crate::domain::error::Result;

/// Row shape as stored in PostgreSQL. Keeping it separate from `Item` means the
/// domain entity carries no `sqlx` derive, so the driver stays in this layer.
#[derive(sqlx::FromRow)]
struct ItemRow {
    id: Uuid,
    name: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<ItemRow> for Item {
    fn from(row: ItemRow) -> Self {
        Item {
            id: row.id,
            name: row.name,
            description: row.description,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

pub struct PgItemService {
    db: PgPool,
}

impl PgItemService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ItemService for PgItemService {
    async fn find_all(&self) -> Result<Vec<Item>> {
        let rows = sqlx::query_as::<_, ItemRow>(
            "select id, name, description, created_at, updated_at
             from items
             order by created_at desc",
        )
        .fetch_all(&self.db)
        .await?;

        Ok(rows.into_iter().map(Item::from).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Item> {
        let row = sqlx::query_as::<_, ItemRow>(
            "select id, name, description, created_at, updated_at
             from items
             where id = $1",
        )
        .bind(id)
        .fetch_one(&self.db)
        .await?;

        Ok(row.into())
    }

    async fn insert(&self, item: Item) -> Result<Item> {
        let row = sqlx::query_as::<_, ItemRow>(
            "insert into items (id, name, description, created_at, updated_at)
             values ($1, $2, $3, $4, $5)
             returning id, name, description, created_at, updated_at",
        )
        .bind(item.id)
        .bind(&item.name)
        .bind(&item.description)
        .bind(item.created_at)
        .bind(item.updated_at)
        .fetch_one(&self.db)
        .await?;

        Ok(row.into())
    }

    async fn update(&self, item: Item) -> Result<Item> {
        let row = sqlx::query_as::<_, ItemRow>(
            "update items
             set name = $2, description = $3, updated_at = $4
             where id = $1
             returning id, name, description, created_at, updated_at",
        )
        .bind(item.id)
        .bind(&item.name)
        .bind(&item.description)
        .bind(item.updated_at)
        .fetch_one(&self.db)
        .await?;

        Ok(row.into())
    }

    async fn delete(&self, id: Uuid) -> Result<Item> {
        let row = sqlx::query_as::<_, ItemRow>(
            "delete from items
             where id = $1
             returning id, name, description, created_at, updated_at",
        )
        .bind(id)
        .fetch_one(&self.db)
        .await?;

        Ok(row.into())
    }
}

#[cfg(all(test, feature = "integration-tests"))]
mod integration_tests {
    use sqlx::{Pool, Postgres};

    use super::*;
    use crate::domain::entities::items::ItemDraft;

    fn draft(name: &str) -> Item {
        ItemDraft::new(name, Some("description"))
            .unwrap()
            .into_new_item(Utc::now())
    }

    #[sqlx::test]
    async fn insert_then_find_all(pool: Pool<Postgres>) {
        let service = PgItemService::new(pool);
        let item = service.insert(draft("widget")).await.unwrap();

        assert_eq!(service.find_all().await.unwrap(), vec![item]);
    }

    #[sqlx::test]
    async fn find_by_id_returns_the_item(pool: Pool<Postgres>) {
        let service = PgItemService::new(pool);
        let item = service.insert(draft("widget")).await.unwrap();

        assert_eq!(service.find_by_id(item.id).await.unwrap(), item);
    }

    #[sqlx::test]
    #[should_panic(expected = "Repository(NotFound)")]
    async fn find_by_id_reports_a_missing_item(pool: Pool<Postgres>) {
        let service = PgItemService::new(pool);
        service.find_by_id(Uuid::new_v4()).await.unwrap();
    }

    #[sqlx::test]
    #[should_panic(expected = "Repository(Conflict)")]
    async fn insert_reports_a_duplicate_id(pool: Pool<Postgres>) {
        let service = PgItemService::new(pool);
        let item = service.insert(draft("widget")).await.unwrap();
        service.insert(item).await.unwrap();
    }

    #[sqlx::test]
    async fn update_changes_the_stored_fields(pool: Pool<Postgres>) {
        let service = PgItemService::new(pool);
        let item = service.insert(draft("widget")).await.unwrap();

        let renamed = Item {
            name: "gadget".to_string(),
            description: None,
            updated_at: Utc::now(),
            ..item.clone()
        };
        let updated = service.update(renamed).await.unwrap();

        assert_eq!(updated.id, item.id);
        assert_eq!(updated.name, "gadget");
        assert_eq!(updated.description, None);
        assert_eq!(updated.created_at, item.created_at);
        assert!(updated.updated_at >= item.updated_at);
    }

    #[sqlx::test]
    #[should_panic(expected = "Repository(NotFound)")]
    async fn update_reports_a_missing_item(pool: Pool<Postgres>) {
        let service = PgItemService::new(pool);
        service.update(draft("widget")).await.unwrap();
    }

    #[sqlx::test]
    async fn delete_removes_the_item(pool: Pool<Postgres>) {
        let service = PgItemService::new(pool);
        let item = service.insert(draft("widget")).await.unwrap();

        assert_eq!(service.delete(item.id).await.unwrap(), item);
        assert_eq!(service.find_all().await.unwrap(), vec![]);
    }

    #[sqlx::test]
    #[should_panic(expected = "Repository(NotFound)")]
    async fn delete_reports_a_missing_item(pool: Pool<Postgres>) {
        let service = PgItemService::new(pool);
        service.delete(Uuid::new_v4()).await.unwrap();
    }
}
