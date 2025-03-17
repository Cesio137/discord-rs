use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Poll {
    pub question: PollMedia,
    pub answers: Vec<PollAnswer>,
    pub expiry: String,
    pub allow_multiselect: bool,
    pub layout_type: i32,
    pub results: Option<PollResult>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollMedia {
    pub text: Option<String>,
    pub emoji: Option<super::emoji::Emoji>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollAnswer {
    pub answer_id: Option<i32>,
    pub poll_media: Option<super::emoji::Emoji>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollResult {
    pub is_finalized: bool,
    pub answer_counts: Vec<PollAnswerCount>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollAnswerCount {
    pub id: i32,
    pub count: i32,
    pub me_voted: bool,
}