use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileId(pub String);

#[derive(Debug, Clone)]
pub struct StoragePath(pub PathBuf);
