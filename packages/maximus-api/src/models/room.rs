use matrix_sdk::ruma::room::RoomType;
use matrix_sdk::RoomState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoomPreview {
    pub room_id: String,
    pub display_name: String,
    pub state: RoomState,
    pub children: Vec<RoomPreview>,
    pub avatar: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub room_id: String,
    pub display_name: String,
    pub room_type: Option<RoomType>,
    pub avatar: Option<String>
}