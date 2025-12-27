use async_trait::async_trait;
use storable_core::{
    errors::StorableError,
    metadata::FileMetadata,
    types::*,
};

#[async_trait]
pub trait FileService {
    async fn upload(
        &self,
        owner: &UserId,
        name: &str,
        mime: &str,
        data: Box<dyn tokio::io::AsyncRead + Send + Unpin>,
    ) -> Result<FileMetadata, StorableError>;

    async fn download(
        &self,
        file_id: &FileId,
    ) -> Result<FileMetadata, StorableError>;

    async fn delete(
        &self,
        file_id: &FileId,
    ) -> Result<(), StorableError>;
}