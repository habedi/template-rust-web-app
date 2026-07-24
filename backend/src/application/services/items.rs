use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::items::Item;
use crate::domain::error::Result;

/// Storage boundary for items. The application layer depends on this trait, and
/// `infrastructure/pg/items.rs` provides the PostgreSQL implementation.
#[async_trait]
pub trait ItemService: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Item>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Item>;
    async fn insert(&self, item: Item) -> Result<Item>;
    async fn update(&self, item: Item) -> Result<Item>;
    async fn delete(&self, id: Uuid) -> Result<Item>;
}

#[cfg(test)]
mockall::mock! {
    pub ItemService {}
    #[async_trait]
    impl ItemService for ItemService {
        async fn find_all(&self) -> Result<Vec<Item>>;
        async fn find_by_id(&self, id: Uuid) -> Result<Item>;
        async fn insert(&self, item: Item) -> Result<Item>;
        async fn update(&self, item: Item) -> Result<Item>;
        async fn delete(&self, id: Uuid) -> Result<Item>;
    }
}
