use serde::{Deserialize, Serialize};
use storable_core::metadata::FileMetadata;
use storable_core::types::{FileId, UserId};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbFile {
    pub id: Thing,    // Internal SurrealDB Record ID (node:uuid)
    pub owner: Thing, // Pointer to user (user:uuid)
    pub name: String,
    pub size: Option<u64>, // Option because folders might not have size
    pub mime: Option<String>,
    pub created_at: std::time::SystemTime,
    pub modified_at: std::time::SystemTime,
}

// The "Glue": Converting DB records to Core Metadata
impl From<DbFile> for FileMetadata {
    fn from(db: DbFile) -> Self {
        Self {
            id: FileId(db.id.id.to_string()),
            // We strip the "user:" prefix for the core UserId if you prefer clean IDs
            owner: UserId(db.owner.id.to_string()),
            name: db.name,
            size: db.size.unwrap_or(0),
            mime: db
                .mime
                .unwrap_or_else(|| "application/octet-stream".to_string()),
            created_at: db.created_at,
            modified_at: db.modified_at,
        }
    }
}
