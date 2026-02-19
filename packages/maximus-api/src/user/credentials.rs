use std::env;
use std::process::exit;
use dioxus::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserCredentials {
    pub homeserver_url: String,
    pub username: String,
    pub password: String,
}

#[server]
pub async fn prefetch_credentials() -> Result<UserCredentials> {
    dotenv().expect("Failed to load .env file");

    let env_vars = env::vars();

    let mut matrix_homeserver_url = None;
    let mut matrix_username = None;
    let mut matrix_password = None;

    for (key, value) in env_vars {
        match key.as_ref() {
            "MATRIX_HOMESERVER_URL" => matrix_homeserver_url = Some(value),
            "MATRIX_USERNAME" => matrix_username = Some(value),
            "MATRIX_PASSWORD" => matrix_password = Some(value),
            _ => {}
        }
    }

    if matrix_homeserver_url.is_none() {
        println!("No MATRIX_HOMESERVER_URL provided");
        exit(1);
    }

    if matrix_username.is_none() {
        println!("No MATRIX_USERNAME provided");
        exit(1);
    }

    if matrix_password.is_none() {
        println!("No MATRIX_PASSWORD provided");
        exit(1);
    }

    Ok(UserCredentials {
        homeserver_url: matrix_homeserver_url.unwrap(),
        username: matrix_username.unwrap(),
        password: matrix_password.unwrap(),
    })
}