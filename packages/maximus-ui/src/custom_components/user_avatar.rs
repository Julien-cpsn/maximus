use dioxus::prelude::*;
use maximus_api::models::session::MatrixAvatar;
use maximus_api::user::avatar::fetch_user_avatar;
use crate::components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};

#[component]
pub fn UserAvatar(size: AvatarImageSize, avatar: Option<MatrixAvatar>) -> Element {
    if let Some(avatar_src) = avatar.as_ref() {
        rsx! {
            InnerUserAvatar {
                size: size,
                server_name: avatar_src.server_name.clone(),
                media_id: avatar_src.media_id.clone(),
            }
        }
    }
    else {
        rsx! {
            Avatar {
                size: size,
                aria_label: "User avatar",
                AvatarFallback { class: "avatar-fallback", "MU" }
            }
        }
    }
}

#[component]
fn InnerUserAvatar(size: AvatarImageSize, server_name: String, media_id: String) -> Element {
    let server_name = use_signal(|| server_name);
    let media_id = use_signal(|| media_id);

    let avatar_data = use_resource(move || async move { fetch_user_avatar(server_name(), media_id()).await.unwrap() });

    match &*avatar_data.read_unchecked() {
        None => rsx! {
            Avatar {
                size: size,
                aria_label: "User avatar",
                AvatarFallback { class: "avatar-fallback", "MU" }
            }
        },
        Some(data) => rsx! {
            Avatar {
                size: size,
                aria_label: "User avatar",
                AvatarImage {
                    src: "data:image/jpeg;base64,{data}",
                    alt: "User avatar",
                },
                AvatarFallback { class: "avatar-fallback", "MU" }
            }
        }
    }
}