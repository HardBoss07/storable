use crate::{errors::StorableError, types::FileId};
use async_trait::async_trait;

#[async_trait]
pub trait StorageBackend: Send + Sync {
    async fn write(
        &self,
        file_id: &FileId,
        data: Box<dyn tokio::io::AsyncRead + Send + Unpin>,
    ) -> Result<(), StorableError>;

    async fn read(
        &self,
        file_id: &FileId,
    ) -> Result<Box<dyn tokio::io::AsyncRead + Send + Unpin>, StorableError>;

    async fn delete(&self, file_id: &FileId) -> Result<(), StorableError>;
}
