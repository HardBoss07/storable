use async_trait::async_trait;
use storable_core::{errors::StorableError, types::*};
use crate::models::*;

#[async_trait]
pub trait UserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<DbUser, StorableError>;
    async fn find_by_username(&self, username: &str) -> Result<DbUser, StorableError>;
}

#[async_trait]
pub trait FileRepository {
    async fn insert(&self, file: DbFile) -> Result<(), StorableError>;
    async fn find_by_id(&self, id: &FileId) -> Result<DbFile, StorableError>;
    async fn delete(&self, id: &FileId) -> Result<(), StorableError>;
    async fn list_for_user(&self, user: &UserId) -> Result<Vec<DbFile>, StorableError>;
}
