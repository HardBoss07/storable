use crate::types::UserId;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub is_admin: bool,
}

pub trait AuthContext {
    fn user(&self) -> &User;
}
