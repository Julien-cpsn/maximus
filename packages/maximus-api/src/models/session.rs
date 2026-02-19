use std::path::PathBuf;
use matrix_sdk::authentication::matrix::MatrixSession;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientSession {
    pub homeserver_url: String,
    pub db_path: PathBuf,
    pub passphrase: String,
}

/// The full session to persist.
#[derive(Debug, Serialize, Deserialize)]
pub struct FullSession {
    pub client_session: ClientSession,
    pub matrix_session: MatrixSession,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub display_name: String,
    pub avatar: Option<MatrixAvatar>,
    pub matrix_session: MatrixSession
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatrixAvatar {
    pub server_name: String,
    pub media_id: String,
}