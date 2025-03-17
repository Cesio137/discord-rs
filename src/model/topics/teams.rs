use serde::{Deserialize, Serialize};
use crate::internal::traits::DiscordTypes;
use crate::model::resources::user::User;

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MembershipState {
    INVITED = 1,
    ACCEPTED = 2
}

impl DiscordTypes for MembershipState {
    fn from(value: u8) -> Self {
        match value { 
            1 => MembershipState::INVITED,
            2 => MembershipState::ACCEPTED,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> u8 {
        match self { 
            MembershipState::INVITED => 1,
            MembershipState::ACCEPTED => 2,
        }
    }
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