use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePath(pub PathBuf);
