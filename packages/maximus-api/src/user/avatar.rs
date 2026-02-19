use std::fs;
use dioxus::prelude::*;
use std::str::FromStr;
use base64::engine::{general_purpose, Engine as _};
use crate::AVATAR_DIR;
use matrix_sdk::ruma::api::client::authenticated_media::get_content_thumbnail;
use matrix_sdk::{Client, OwnedServerName};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use dioxus::fullstack::body::Bytes;
use matrix_sdk::ruma::UInt;
use crate::user::login::get_client;

#[server]
pub async fn fetch_user_avatar(server_name: String, media_id: String) -> anyhow::Result<String> {
    let avatar_path = AVATAR_DIR.join(format!("{media_id}.jpg"));

    if !avatar_path.exists() {
        let server_name = OwnedServerName::from_str(server_name.as_str())?;
        let request = get_content_thumbnail::v1::Request::new(media_id, server_name, UInt::new_saturating(500), UInt::new_saturating(500));

        let client = get_client()?;
        let response = client.send(request).await?;

        let mut avatar_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&avatar_path)
            .expect("Could not create avatar file");

        avatar_file.write_all(&response.file)?;
    }

    let data = fs::read(&avatar_path).unwrap();

    Ok(general_purpose::STANDARD.encode(&data))
}