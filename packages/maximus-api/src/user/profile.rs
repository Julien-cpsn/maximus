use matrix_sdk::ruma::api::client::profile::{get_profile, DisplayName};
use matrix_sdk::ruma::OwnedUserId;
use matrix_sdk::Client;

pub async fn fetch_display_name(client: &Client, user_id: OwnedUserId) -> anyhow::Result<String>{
    let request = get_profile::v3::Request::new(user_id.clone());
    let response = client.send(request).await?;

    let display_name = response.get_static::<DisplayName>()?;

    Ok(display_name.unwrap_or(user_id.to_string()))
}