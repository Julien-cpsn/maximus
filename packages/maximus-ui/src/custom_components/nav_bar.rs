use dioxus::prelude::*;
use maximus_api::user::login::{is_logged, logout};
use maximus_api::user::session::get_user_session;
use crate::components::avatar::AvatarImageSize;
use crate::components::button::{Button, ButtonVariant};
use crate::components::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
use crate::custom_components::dialogs::profile_dialog::ProfileDialog;
use crate::custom_components::user_avatar::UserAvatar;
use crate::Route;

#[component]
pub fn Navbar() -> Element {
    let nav = navigator();

    let is_logged = use_server_future(move || async move { is_logged().await.unwrap() })?;

    if is_logged().unwrap() == false {
        nav.replace(Route::Login);

        rsx! {
            div {
                "Loading..."
            }
        }
    }
    else {
        let mut open_profile = use_signal(|| false);

        let logout = move |_| async move {
            logout().await.unwrap();
            nav.push(Route::Login);
        };

        let mut user_session_resource = use_server_future(move || async move { get_user_session().await.unwrap() })?;
        let user_session = use_signal(|| user_session_resource.value().unwrap());

        rsx! {
            div {
                class: "fixed bottom-3 left-2 w-100 h-26 p-4 rounded-lg bg-neutral-900",
                div {
                    class: "flex flex-row justify-between gap-2 h-full",
                    div {
                        class: "flex content-around mx-1 my-auto",
                        UserAvatar {
                            size: AvatarImageSize::Medium,
                            avatar: user_session().avatar.clone(),
                        }
                    },
                    div {
                        class: "flex flex-col w-full my-auto",
                        b {
                            "{user_session().display_name}"
                        },
                        p {
                            class: "font-extralight",
                            "{user_session().matrix_session.meta.user_id}"
                        },
                        p {
                            class: "font-extralight",
                            "{user_session().matrix_session.meta.device_id}"
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
            },

            ProfileDialog {
                open_profile: open_profile,
                user_session: user_session,
            },

            Outlet::<Route> {}
        }
    }
}