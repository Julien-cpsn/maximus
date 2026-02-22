use std::time::Duration;
use dioxus::fullstack::{use_websocket, WebSocketOptions};
use dioxus::prelude::*;
use dioxus_primitives::ContentSide;
use dioxus_sdk_time::sleep;
use maximus_api::rooms::list::get_user_rooms;
use maximus_api::user::healthcheck::healthcheck_ws;
use maximus_api::user::login::logout;
use maximus_api::user::session::get_user_session;
use maximus_shared_models::events::healthcheck::{HealthCheckEvent, HealthCheckEventResponse};
use maximus_shared_models::room::RoomPreview;
use crate::dioxus_components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};
use crate::dioxus_components::button::{Button, ButtonRadius, ButtonVariant};
use crate::dioxus_components::separator::Separator;
use crate::custom_components::dialogs::profile_dialog::ProfileDialog;
use crate::custom_components::user_avatar::UserAvatar;
use crate::dioxus_components::badge::{Badge, BadgeVariant};
use crate::dioxus_components::tooltip::{Tooltip, TooltipContent, TooltipTrigger};
use crate::custom_components::utils::spinner::spinner::{Spinner, SpinnerSize};
use crate::{Route, ROOMS};

#[component]
pub fn MainLayout() -> Element {
    let nav = navigator();

    let mut is_logged = use_signal(|| None);

    let mut healthcheck_socket = use_websocket(|| {
        let options = WebSocketOptions::new().with_automatic_reconnect();
        healthcheck_ws(options)
    });

    use_future(move || async move {
        while let Ok(event) = healthcheck_socket.recv().await {
            debug!("{}", event);
            match event {
                HealthCheckEventResponse::LoginStatus(is_logged_in) => is_logged.set(Some(is_logged_in))
            }
        }
    });

    use_future(move || async move {
        loop {
            sleep(Duration::from_secs(5)).await;
            let _ = healthcheck_socket.send(HealthCheckEvent::IsLoggedIn).await;
        }
    });

    let is_logged = is_logged.read().clone();

    if let Some(is_logged) = is_logged {
        if !is_logged {
            nav.replace(Route::Login);
            return rsx! {
                div {
                    "Not logged in, returning to logging page..."
                }
            }
        }

        return rsx! {
            div {
                class: "flex flex-row w-screen h-screen",
                style: "background-color: var(--primary-color-2)",

                ServerPart {},

                div {
                    class: "w-full",
                    Outlet::<Route> {}
                },
            }

            ProfilePart {}
        }
    }
    else {
        return rsx! {
            div {
                class: "flex flex-col justify-center items-center w-screen h-screen",
                div {
                    class: "flex flex-col gap-2 justify-center items-center",
                    Spinner {
                        size: SpinnerSize::Large
                    },
                    p { "Loading..." }
                }
            }
        }
    }
}

#[component]
fn ServerPart() -> Element {
    let mut rooms_loader = use_loader(move || async move { get_user_rooms().await })?;

    if rooms_loader.loading() {
        return rsx! {
            Spinner {
                size: SpinnerSize::Medium
            }
        }
    }
    else if ROOMS.read().is_none() {
        *ROOMS.write() = Some(rooms_loader.take());
    }

    let rooms = &*ROOMS.read();
    let rooms = rooms.as_ref().unwrap();
    let unread_dms = rooms.dms.iter().filter(|(_, r)| r.unread_messages_count > 0);

    rsx! {
        div {
            class: "w-17 h-full px-2 py-3 border-r-1 border-r-white/10",
            div {
                class: "flex flex-col gap-2",
                Link {
                    style: "height: 3rem",
                    to: Route::Home,
                    Button {
                        style: "height: 3rem",
                        variant: ButtonVariant::Ghost,
                        radius: ButtonRadius::Circle,
                        img {
                            class: "w-full h-full",
                            style: "padding: 5px; border-radius: 50%; background-color: var(--primary-color-4)",
                            src: asset!("/assets/images/discussion.svg"),
                        }
                    }
                },

                for (_, room) in unread_dms {
                    ServerButton {
                        room: room.clone()
                    }
                }

                Separator {
                    horizontal: true,
                    decorative: true
                },

                for (room_id, room) in rooms.servers.iter() {
                    ServerButton {
                        key: "{room_id}",
                        room: room.clone()
                    }
                }
            }
        }
    }
}

#[component]
fn ServerButton(room: RoomPreview) -> Element {
    rsx! {
        Tooltip {
            TooltipTrigger {
                Link {
                    to: Route::Server {
                        server_id: room.room_id
                    },
                    Button {
                        style: "height: 3rem",
                        variant: ButtonVariant::Ghost,
                        radius: ButtonRadius::Circle,
                        if let Some(avatar) = &room.avatar {
                            Avatar {
                                size: AvatarImageSize::Medium,
                                aria_label: "Room avatar",
                                AvatarImage {
                                    src: "data:image/jpeg;base64,{avatar}",
                                    alt: "Room avatar",
                                },
                                AvatarFallback { class: "avatar-fallback", "RA" }
                            }
                        }
                        else {
                            Avatar {
                                size: AvatarImageSize::Medium,
                                aria_label: "Room avatar",
                                AvatarFallback { class: "avatar-fallback", "RA" }
                            }
                        }
                        div {
                            class: "absolute bottom-0 right-0",
                            ServerUnreadMessageCount {
                                unread_message_count: room.unread_messages_count
                            }
                        }
                    }
                }
            },
            TooltipContent {
                side: ContentSide::Right,
                span { "{room.display_name}" }
            }
        }
    }
}

#[component]
fn ProfilePart() -> Element {
    let nav = navigator();

    let user_session_loader = use_loader(move || async move { get_user_session().await })?;
    let mut open_profile = use_signal(|| false);

    let logout = move |_| async move {
        logout().await.unwrap();
        nav.push(Route::Login);
    };

    rsx! {
        match user_session_loader.loading() {
            true => rsx! {
                p { "Loading..." }
            },
            false => {
                let user_session = user_session_loader.read();

                rsx! {
                    div {
                        class: "fixed bottom-0 left-0 w-288px h-26 px-2 py-4 rounded-t-lg bg-neutral-900",
                        style: "z-index: 50",
                        div {
                            class: "flex flex-row justify-between gap-2 h-full",
                            div {
                                class: "flex content-around my-auto",
                                UserAvatar {
                                    size: AvatarImageSize::Medium,
                                    avatar_data: user_session.avatar.clone(),
                                }
                            },
                            div {
                                class: "flex flex-col w-full my-auto",
                                b {
                                    "{user_session.display_name}"
                                },
                                p {
                                    class: "font-extralight",
                                    "{user_session.matrix_session.meta.user_id}"
                                },
                                p {
                                    class: "font-extralight",
                                    "{user_session.matrix_session.meta.device_id}"
                                },
                            },
                            div {
                                class: "flex flex-col place-content-between h-full ml-4 my-auto",
                                Button {
                                    class: "text-xs p-2",
                                    variant: ButtonVariant::Secondary,
                                    onclick: move |_| open_profile.set(true),
                                    "Profile"
                                }
                                Button {
                                    class: "text-xs p-2",
                                    variant: ButtonVariant::Destructive,
                                    onclick: logout,
                                    "Logout"
                                }
                            }
                        }
                    }
                }
            }
        },

        ProfileDialog {
            open_profile: open_profile,
            user_session: ReadSignal::new(user_session_loader),
        },
    }
}

#[component]
fn ServerUnreadMessageCount(unread_message_count: u64) -> Element {
    rsx! {
        if unread_message_count == 0 {

        }
        else if unread_message_count > 99 {
            Badge {
                variant: BadgeVariant::Destructive,
                "{unread_message_count}"
            }
        }
        else {
            Badge {
                variant: BadgeVariant::Destructive,
                "{unread_message_count}"
            }
        }
    }
}