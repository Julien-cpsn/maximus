use std::fs;
use std::str::FromStr;
use base64::engine::{general_purpose, Engine as _};
use matrix_sdk::ruma::api::client::authenticated_media::get_content_thumbnail;
use matrix_sdk::{Client, OwnedServerName};
use std::fs::OpenOptions;
use std::io::Write;
use matrix_sdk::ruma::api::client::profile::{get_profile, AvatarUrl};
use matrix_sdk::ruma::{OwnedUserId, UInt};
use crate::consts::AVATAR_DIR;
use crate::user::login::get_client;

pub async fn fetch_user_avatar(client: &Client, user_id: OwnedUserId) -> anyhow::Result<Option<String>> {
    let request = get_profile::v3::Request::new(user_id);
    let response = client.send(request).await?;
    let avatar_url = response.get_static::<AvatarUrl>()?;

    if avatar_url.is_none() {
        return Ok(None);
    }

    let avatar_url = avatar_url.unwrap();
    let (server_name, media_id) = avatar_url.parts()?;

    let avatar_path = AVATAR_DIR.join(format!("{media_id}.jpg"));

    if !avatar_path.exists() {
        let server_name = OwnedServerName::from_str(server_name.as_str())?;
        let request = get_content_thumbnail::v1::Request::new(media_id.to_string(), server_name, UInt::new_saturating(500), UInt::new_saturating(500));

        let client = get_client()?;
        let response = client.send(request).await?;

        let mut avatar_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&avatar_path)
            .expect("Could not create avatar file");

        avatar_file.write_all(&response.file)?;
    }

    let data = fs::read(&avatar_path)?;

    Ok(Some(general_purpose::STANDARD.encode(&data)))
}