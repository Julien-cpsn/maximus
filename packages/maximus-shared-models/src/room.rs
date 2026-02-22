use matrix_sdk::RoomState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoomPreview {
    pub room_id: String,
    pub display_name: String,
    pub state: RoomState,
    pub room_type: RoomType,
    pub children: Vec<RoomPreview>,
    pub avatar: Option<String>,
    pub unread_messages_count: u64
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoomType {
    Space,
    Call,
    Text
}