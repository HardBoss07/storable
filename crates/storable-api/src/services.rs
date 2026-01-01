use async_trait::async_trait;
use std::sync::Arc;
use storable_core::{errors::StorableError, metadata::FileMetadata, types::*};
use storable_db::repositories::FileRepository;

#[async_trait]
pub trait FileService {
    async fn upload(
        &self,
        owner: &UserId,
        name: &str,
        mime: &str,
        data: Box<dyn tokio::io::AsyncRead + Send + Unpin>,
    ) -> Result<FileMetadata, StorableError>;

    async fn download(&self, file_id: &FileId) -> Result<FileMetadata, StorableError>;

    async fn delete(&self, file_id: &FileId) -> Result<(), StorableError>;
}

pub struct StorableFileService {
    repo: Arc<FileRepository>,
}

impl StorableFileService {
    pub fn new(repo: FileRepository) -> Self {
        Self {
            repo: Arc::new(repo),
        }
    }
}

#[async_trait]
impl FileService for StorableFileService {
    async fn upload(
        &self,
        owner: &UserId,
        name: &str,
        mime: &str,
        mut _data: Box<dyn tokio::io::AsyncRead + Send + Unpin>,
    ) -> Result<FileMetadata, StorableError> {
        // TODO: Implement fs::save logic here to write bytes to disk
        // Then perform db insert
        Err(StorableError::Internal)
    }

    async fn download(&self, file_id: &FileId) -> Result<FileMetadata, StorableError> {
        self.repo
            .get_file_by_id(file_id)
            .await
            .map_err(|_| StorableError::Internal)?
            .ok_or(StorableError::NotFound)
    }

    async fn delete(&self, file_id: &FileId) -> Result<(), StorableError> {
        self.repo
            .delete_file(file_id)
            .await
            .map_err(|_| StorableError::Internal)
    }
}
