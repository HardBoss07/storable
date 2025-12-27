use crate::types::{FileId, UserId};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub id: FileId,
    pub owner: UserId,
    pub name: String,
    pub size: u64,
    pub mime: String,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
}
