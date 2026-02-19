use matrix_sdk::ruma::api::client::profile::{get_profile, AvatarUrl, DisplayName};
use matrix_sdk::ruma::OwnedUserId;
use matrix_sdk::{Client, OwnedServerName};

pub async fn fetch_profile(client: &Client, user_id: OwnedUserId) -> anyhow::Result<(String, Option<(OwnedServerName, String)>)>{
    let request = get_profile::v3::Request::new(user_id.clone());
    let response = client.send(request).await?;

    let avatar_url = match response.get_static::<AvatarUrl>()? {
        Some(mxc_avatar_url) => {
            let (server_name, media_id) = mxc_avatar_url.parts()?;
            Some((server_name.to_owned(), media_id.to_string()))
        },
        None => None
    };
    let display_name = response.get_static::<DisplayName>()?;

    Ok((display_name.unwrap_or(user_id.to_string()), avatar_url))
}