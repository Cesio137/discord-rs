use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::model::resources::user::User;

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum MembershipState {
    INVITED = 1,
    ACCEPTED = 2
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Team {
    pub icon: Option<String>,
    pub id: String,
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamMember {
    pub membership_state: MembershipState,
    pub team_id: String,
    pub user: User,
    pub role: String,
    pub owner_user_id: String
}