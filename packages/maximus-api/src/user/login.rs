use std::fs;
use anyhow::anyhow;
use dioxus::html::completions::CompleteWithBraces::tr;
use dioxus::prelude::*;
use matrix_sdk::authentication::matrix::MatrixSession;
use matrix_sdk::{AuthSession, Client};
use once_cell::unsync::Lazy;
use parking_lot::{Mutex};
use crate::{DATABASE_DIR, DATA_DIR, SESSION_FILE_PATH};
use crate::models::session::FullSession;
use crate::user::credentials::UserCredentials;
use crate::user::session::{build_client, get_session, save_full_session, sync};

thread_local! {
    pub static MATRIX_CLIENT: Lazy<Mutex<Option<Client>>> = Lazy::new(|| Mutex::new(None));
}

pub fn get_client() -> Result<Client, ServerFnError> {
    MATRIX_CLIENT.with(|client|
        match &*client.lock() {
            Some(client) => Ok(client.clone()),
            None => Err(ServerFnError::ServerError {
                message: String::from("NOT LOGGED IN"),
                code: 401,
                details: None,
            })
        }
    )
}

#[server]
pub async fn login(credentials: UserCredentials) -> Result<(), ServerFnError> {
    let (client, sync_token) = match SESSION_FILE_PATH.exists() {
        true => restore_session().await?,
        false => {
            let client = new_session(&credentials).await?;
            (client, None)
        },
    };

    sync(&client, sync_token).await?;

    MATRIX_CLIENT.with(|new_client| {
        *new_client.lock() = Some(client);
    });

    Ok(())
}

async fn new_session(credentials: &UserCredentials) -> Result<Client, ServerFnError> {
    info!("Creating new session");

    let (client, passphrase) = build_client(&credentials.homeserver_url).await?;

    if let Err(error) = client.matrix_auth()
        .login_username(&credentials.username, &credentials.password)
        .initial_device_display_name("Maximus client")
        .await
    {
        return Err(ServerFnError::ServerError {
            message: error.to_string(),
            code: 400,
            details: None
        })
    }

    let matrix_session = get_session(&client)?;

    save_full_session(credentials.homeserver_url.clone(), passphrase, matrix_session);

    info!("New session created");

    Ok(client)
}

async fn try_restore_session() -> anyhow::Result<bool> {
    if SESSION_FILE_PATH.exists() {
        let (client, sync_token) = restore_session().await?;

        sync(&client, sync_token).await?;

        MATRIX_CLIENT.with(|new_client| {
            *new_client.lock() = Some(client);
        });

        Ok(true)
    }
    else {
        Ok(false)
    }
}

async fn restore_session() -> anyhow::Result<(Client, Option<String>)> {
    println!("Previous session found");

    let serialized_session: String = fs::read_to_string(SESSION_FILE_PATH.as_path())?;
    let full_session: FullSession = serde_json::from_str(&serialized_session)?;

    let mut client_builder = Client::builder().homeserver_url(&full_session.client_session.homeserver_url);

    #[cfg(not(target_arch = "wasm32"))]
    {
        client_builder = client_builder.sqlite_store(full_session.client_session.db_path, Some(&full_session.client_session.passphrase));
    }

    let client = client_builder.build().await?;

    println!("Restoring session for \"{}\"", full_session.matrix_session.meta.user_id);

    client.restore_session(full_session.matrix_session).await.expect("Session already restored or logged in");

    Ok((client, full_session.sync_token))
}

#[server]
pub async fn is_logged() -> Result<bool> {
    let mut is_already_logged = false;

    MATRIX_CLIENT.with(|cell| {
        if let Some(client) = cell.lock().as_ref() {
            if client.session().is_some() {
                is_already_logged = true
            }
        }
    });

    if is_already_logged {
        Ok(true)
    }
    else {
        try_restore_session().await.map_err(|e| e.into())
    }
}

#[server]
pub async fn logout() -> Result<()> {
    let client = MATRIX_CLIENT.with(|cell| cell.lock().take());

    if let Some(client) = client {
        let _ = client.logout().await;
    }

    fs::remove_file(SESSION_FILE_PATH.as_path())?;

    for file in fs::read_dir(DATABASE_DIR.as_path())? {
        if let Ok(file) = file {
            fs::remove_file(file.path())?;
        }
    }

    return Ok(())
}