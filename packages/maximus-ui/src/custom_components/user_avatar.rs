use dioxus::prelude::*;
use crate::components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};

#[component]
pub fn UserAvatar(size: AvatarImageSize, avatar_data: Option<String>) -> Element {
    match &avatar_data {
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
        },
        None => rsx! {
            Avatar {
                size: size,
                aria_label: "User avatar",
                AvatarFallback { class: "avatar-fallback", "MU" }
            }
        }
    }
}