use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ban {
    pub reason: Option<String>,
    pub user: super::super::user::User,
}