use storable_core::types::{FileId, UserId};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct DbUser {
    pub id: UserId,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone)]
pub struct DbFile {
    pub id: FileId,
    pub owner: UserId,
    pub name: String,
    pub size: u64,
    pub mime: String,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
}
