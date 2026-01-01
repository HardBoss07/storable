use crate::models::DbFile;
use storable_core::metadata::FileMetadata;
use storable_core::types::{FileId, UserId};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub struct FileRepository {
    db: Surreal<Client>,
}

impl FileRepository {
    pub fn new(db: Surreal<Client>) -> Self {
        Self { db }
    }

    pub async fn list_all_files(&self, owner_id: &UserId) -> Result<Vec<FileMetadata>, String> {
        // We use the "user" table prefix for the record ID
        let owner_record = format!("user:{}", owner_id.0);

        let mut response = self
            .db
            .query("SELECT * FROM node WHERE owner = $owner AND kind = 'file'")
            .bind(("owner", owner_record))
            .await
            .map_err(|e| e.to_string())?;

        let db_files: Vec<DbFile> = response.take(0).map_err(|e| e.to_string())?;

        // Map the DB internal models to your Core Metadata
        Ok(db_files.into_iter().map(FileMetadata::from).collect())
    }

    pub async fn get_file_by_id(&self, file_id: &FileId) -> Result<Option<FileMetadata>, String> {
        let node_record = format!("node:{}", file_id.0);

        let mut response = self
            .db
            .query("SELECT * FROM node WHERE id = $id AND kind = 'file' LIMIT 1")
            .bind(("id", node_record))
            .await
            .map_err(|e| e.to_string())?;

        let db_file: Option<DbFile> = response.take(0).map_err(|e| e.to_string())?;
        Ok(db_file.map(FileMetadata::from))
    }

    pub async fn delete_file(&self, file_id: &FileId) -> Result<(), String> {
        // Build the record id string (e.g. "node:<id>") and delete by that resource.
        let record = format!("node:{}", file_id.0);

        // Delete accepts types that implement `IntoResource`, `String`/`&str` are supported.
        let _: Vec<DbFile> = self.db.delete(record).await.map_err(|e| e.to_string())?;

        Ok(())
    }
}
