use std::time::Duration;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use maximus_api::user::credentials::UserCredentials;
use maximus_api::user::login::login;
use crate::components::button::Button;
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::toast::ToastProvider;
use crate::Route;

#[component]
pub fn Login() -> Element {
    let mut homeserver_url = use_signal(String::new);
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);

    use_resource(move || async move {
        let credentials = maximus_api::user::credentials::prefetch_credentials().await.unwrap();

        homeserver_url.set(credentials.homeserver_url.clone());
        username.set(credentials.username.clone());
        password.set(credentials.password.clone());
    });

    rsx! {
        ToastProvider {
            div {
                class: "flex justify-center place-center items-center min-h-screen",
                div {
                    class: "flex flex-col gap-10 w-sm",
                    div {
                        class: "flex flex-col justify-center items-center w-full h-18 rounded-lg bg-linear-to-bl from-main-500 to-main-900",
                        h1 {
                            class: "flex flex-row gap-0.25 text-4xl text-center",
                            p {
                                class: "font-semibold",
                                "M"
                            },
                            p {
                                class: "font-extralight",
                                "aximus"
                            }
                        },
                    },
                    div {
                        class: "flex flex-col gap-4 w-full mx-auto",
                        div {
                            class: "flex flex-col gap-2",
                            Label { html_for: "homeserver_url", "Homeserver URL"}
                            Input {
                                id: "homeserver_url",
                                placeholder: "https://matrix.org",
                                oninput: move |e: FormEvent| homeserver_url.set(e.value()),
                                value: homeserver_url
                            }
                        },
                        div {
                            class: "flex flex-col gap-2",
                            Label { html_for: "username","Username"}
                            Input {
                                id: "username",
                                placeholder: "username",
                                oninput: move |e: FormEvent| username.set(e.value()),
                                value: username
                            }
                        },
                        div {
                            class: "flex flex-col gap-2",
                            Label { html_for: "password", "Password"}
                            Input {
                                id: "password",
                                type: "password",
                                oninput: move |e: FormEvent| password.set(e.value()),
                                value: password
                            }
                        },
                    },
                    div {
                        class: "mx-auto",
                        LoginButton {
                            class: "px-8 py-2",
                            homeserver_url: homeserver_url,
                            username: username,
                            password: password
                        }
                    }
                }
            },
        }
    }
}

#[component]
fn LoginButton(
    #[props(extends=GlobalAttributes)]
    #[props(extends=button)]
    attributes: Vec<Attribute>,
    homeserver_url: Signal<String>,
    username: Signal<String>,
    password: Signal<String>
) -> Element {
    let nav = navigator();
    let toast_api = use_toast();

    let login = move |_| async move {
        let credentials = UserCredentials {
            homeserver_url: homeserver_url.read().clone(),
            username: username.read().clone(),
            password: password.read().clone(),
        };

        match login(credentials).await {
            Ok(_) => {
                nav.push(Route::Home);
            },
            Err(error) => {
                if let ServerFnError::ServerError { message, ..} = error {
                    toast_api.error(
                        String::from("Error"),
                        ToastOptions::new()
                            .description(message.as_str())
                            .duration(Duration::from_secs(15))
                            .permanent(false),
                    );
                }
            }
        }
    };

    rsx! {
        Button {
            attributes: attributes,
            onclick: login,
            "Connect"
        },
    }
}