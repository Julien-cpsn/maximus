use dioxus::prelude::*;
use maximus_api::models::session::UserSession;
use crate::components::avatar::AvatarImageSize;
use crate::components::button::{Button, ButtonVariant};
use crate::components::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
use crate::custom_components::copy_button::CopyButton;
use crate::custom_components::user_avatar::UserAvatar;

#[component]
pub fn ProfileDialog(open_profile: Signal<bool>, user_session: ReadSignal<UserSession>) -> Element {
    let user_session_read = user_session.read();
    let display_name = user_session_read.display_name.as_str();
    let user_id = user_session_read.matrix_session.meta.user_id.as_str();
    let device_id = user_session_read.matrix_session.meta.device_id.as_str();
    let access_token = user_session_read.matrix_session.tokens.access_token.as_str();
    let avatar = user_session_read.avatar.clone();

    rsx! {
        DialogRoot {
            open: open_profile(),
            on_open_change: move |v| open_profile.set(v),
            DialogContent {
                DialogTitle {
                    "Profile"
                },
                DialogDescription {
                    div {
                        class: "flex flex-col gap-4 mt-2 mb-4",
                        div {
                            class: "flex flex-col items-center gap-1 mb-4",
                            b { "Avatar" },
                            UserAvatar {
                                size: AvatarImageSize::ExtraLarge,
                                avatar_data: avatar,
                            }
                        },
                        div {
                            class: "flex flex-col items-start",
                            b { "Display name" },
                            div {
                                class: "flex flex-row justify-between items-center w-full",
                                p {
                                    "{display_name}"
                                },
                                CopyButton {
                                    text_to_copy: display_name.to_string()
                                }
                            }
                        },
                        div {
                            class: "flex flex-col items-start",
                            b { "Username" },
                            div {
                                class: "flex flex-row justify-between items-center w-full",
                                p {
                                    "{user_id}"
                                },
                                CopyButton {
                                    text_to_copy: user_id.to_string()
                                }
                            }
                        },
                        div {
                            class: "flex flex-col items-start",
                            b { "Device ID" },
                            div {
                                class: "flex flex-row justify-between items-center w-full",
                                p {
                                    "{device_id}"
                                },
                                CopyButton {
                                    text_to_copy: device_id.to_string()
                                }
                            }
                        },
                        div {
                            class: "flex flex-col items-start",
                            b { "Access token" },
                            div {
                                class: "flex flex-row justify-between items-center w-full",
                                p {
                                    "{access_token}"
                                },
                                CopyButton {
                                    text_to_copy: access_token.to_string()
                                }
                            }
                        },
                    }
                },
                div {
                    class: "flex flex-row place-content-end",
                    Button {
                        class: "p-2",
                        variant: ButtonVariant::Secondary,
                        tabindex: if open_profile() { "0" } else { "-1" },
                        onclick: move |_| open_profile.set(false),
                        "Close"
                    },
                }
            }
        }
    }
}