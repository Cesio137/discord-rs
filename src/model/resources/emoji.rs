use serde::{Deserialize, Serialize};

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Emoji {
    pub id: Option<String>,
    pub name: Option<String>,
    pub roles: Option<Vec<String>>,
    pub user: Option<super::user::User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}