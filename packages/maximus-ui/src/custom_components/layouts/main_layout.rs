use dioxus::prelude::*;
use dioxus_primitives::ContentSide;
use maximus_api::rooms::list::get_user_rooms;
use maximus_api::user::login::{is_logged, logout};
use maximus_api::user::session::get_user_session;
use crate::components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};
use crate::components::button::{Button, ButtonRadius, ButtonVariant};
use crate::components::separator::Separator;
use crate::custom_components::dialogs::profile_dialog::ProfileDialog;
use crate::custom_components::user_avatar::UserAvatar;
use crate::{Route, ROOMS};
use crate::components::tooltip::{Tooltip, TooltipContent, TooltipTrigger};
use crate::custom_components::utils::spinner::spinner::{Spinner, SpinnerSize};

#[component]
pub fn MainLayout() -> Element {
    let nav = navigator();

    let is_logged = use_loader(move || async move { is_logged().await })?;

    if is_logged.loading() {
        rsx! {
            div {
                "Loading..."
            }
        }
    }
    else if &*is_logged.read() == &false {
        nav.replace(Route::Login);
        return rsx! {
            div {
                "Not logged in, returning to logging page..."
            }
        }
    }
    else {


        rsx! {
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

    rsx! {
        div {
            class: "w-17 h-full px-2 py-3",
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

                Separator {
                    horizontal: true,
                    decorative: true
                },

                for (room_id, room) in rooms.servers.iter() {
                    Tooltip {
                        TooltipTrigger {
                            Link {
                                to: Route::Server {
                                    server_id: room_id.clone()
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
        }
    }
}

#[component]
fn ProfilePart() -> Element {
    let nav = navigator();

    let user_session_loader = use_loader(move || async move { get_user_session().await })?;
    let mut open_profile = use_signal(|| false);

    /*
    let logout = move |_| async move {
        logout().await.unwrap();
        nav.push(Route::Login);
    };*/

    rsx! {
        match user_session_loader.loading() {
            true => rsx! {
                p { "Loading..." }
            },
            false => {
                let user_session = user_session_loader.read();

                rsx! {
                    div {
                        class: "fixed bottom-3 left-2 w-270px h-26 p-4 rounded-lg bg-neutral-900",
                        style: "z-index: 50",
                        div {
                            class: "flex flex-row justify-between gap-2 h-full",
                            div {
                                class: "flex content-around mx-1 my-auto",
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
                            /*
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
                            }*/
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