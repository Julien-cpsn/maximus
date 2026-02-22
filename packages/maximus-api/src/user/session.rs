use std::fs;
use base64::engine::{general_purpose, Engine as _};
use dioxus::prelude::*;
use matrix_sdk::authentication::matrix::MatrixSession;
use matrix_sdk::{AuthSession, Client, LoopCtrl};
use matrix_sdk::config::SyncSettings;
use matrix_sdk::media::MediaFormat;
use matrix_sdk::ruma::api::client::filter::FilterDefinition;
use rand::distr::Alphanumeric;
use rand::{rng, RngExt};
use crate::{DATABASE_DIR, SESSION_FILE_PATH};
use crate::models::session::{ClientSession, FullSession, UserSession};
use crate::user::profile::fetch_display_name;
use crate::user::login::get_client;


pub async fn build_client(homeserver_url: &str) -> Result<(Client, String), ServerFnError> {
    let passphrase: String = (&mut rng()).sample_iter(&Alphanumeric).take(32).map(char::from).collect();

    let mut client_builder = Client::builder().homeserver_url(homeserver_url);

    #[cfg(not(target_arch = "wasm32"))]
    {
        client_builder = client_builder.sqlite_store(DATABASE_DIR.as_path(), Some(&passphrase));
    }

    let matrix_client = match client_builder.build().await {
        Ok(client) => client,
        Err(error) => return Err(ServerFnError::ServerError {
            message: error.to_string(),
            code: 400,
            details: None
        }),
    };

    Ok((matrix_client, passphrase))
}

pub fn get_session(client: &Client) -> Result<MatrixSession, ServerFnError> {
    match client.session() {
        Some(session) => match session {
            AuthSession::Matrix(matrix_session) => Ok(matrix_session),
            _ => unimplemented!(),
        },
        None =>  Err(ServerFnError::ServerError {
            message: String::from("NOT LOGGED IN"),
            code: 401,
            details: None,
        })
    }
}

pub fn save_full_session(homeserver_url: String, passphrase: String, matrix_session: MatrixSession) {
    info!("Saving session");

    let full_session = FullSession {
        client_session: ClientSession {
            homeserver_url,
            db_path: DATABASE_DIR.clone(),
            passphrase,
        },
        matrix_session,
        sync_token: None,
    };

    let serialized_session = serde_json::to_string(&full_session).expect("Could not serialize session");

    fs::write(SESSION_FILE_PATH.as_path(), serialized_session).expect("Could not write session");

    info!("Session saved");
}

#[server]
pub async fn get_user_session() -> Result<UserSession> {
    let client = get_client()?;
    let matrix_session = get_session(&client)?;

    debug!("access token: {}", matrix_session.tokens.access_token);

    let display_name = fetch_display_name(&client, matrix_session.meta.user_id.to_owned()).await?;
    let avatar = client.account().get_avatar(MediaFormat::File).await?;

    Ok(UserSession {
        display_name,
        avatar: avatar.map(|data| general_purpose::STANDARD.encode(&data)),
        matrix_session,
    })
}

pub async fn sync(client: &Client, initial_sync_token: Option<String>) -> anyhow::Result<()> {
    info!("Syncing session");

    let filter = FilterDefinition::with_lazy_loading();

    let mut sync_settings = SyncSettings::default().filter(filter.into());

    if let Some(sync_token) = initial_sync_token {
        sync_settings = sync_settings.token(sync_token);
    }

    loop {
        match client.sync_once(sync_settings.clone()).await {
            Ok(response) => {
                sync_settings = sync_settings.token(response.next_batch.clone());
                persist_sync_token(response.next_batch);
                break;
            }
            Err(error) => {
                warn!("An error occurred during initial sync: {error}");
                warn!("Trying again…");
            }
        }
    }

    info!("Session synced");

    Ok(())
}

fn persist_sync_token(sync_token: String) {
    let serialized_session: String = fs::read_to_string(SESSION_FILE_PATH.as_path()).expect("Could not read session file");
    let mut full_session: FullSession = serde_json::from_str(&serialized_session).expect("Could not deserialize session");

    full_session.sync_token = Some(sync_token);
    let serialized_session = serde_json::to_string(&full_session).expect("Could not serialize session");
    fs::write(SESSION_FILE_PATH.as_path(), serialized_session).expect("Could not write session");
}