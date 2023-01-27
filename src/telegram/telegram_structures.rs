use serde::Deserialize;

#[derive(Deserialize)]
pub struct TgGetMeResult {
    pub id: u64,
    pub is_bot: bool,
    pub first_name: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct TgResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub error_code: Option<u64>,
    pub description: Option<String>,
}

#[derive(Deserialize, PartialEq)]
pub struct TgUser {
    pub id: u64,
}
#[derive(Deserialize)]
pub struct TgMessage {
    pub message_id: u64,
    pub from: Option<TgUser>,
    pub text: Option<String>,
}

#[derive(Deserialize)]
pub struct TgChat {
    pub id: i64,
}

#[derive(Deserialize)]
pub struct TgCallbackData {
    pub data: String,
    pub from: TgUser,
    pub message: TgMessage,
}

#[derive(Deserialize)]
pub struct TgUpdate {
    pub update_id: u64,
    pub message: Option<TgMessage>,
}

#[derive(PartialEq)]
pub enum Control {
    Stop,
    Play,
    Nothing,
}
