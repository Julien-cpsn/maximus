use std::collections::HashMap;
use dioxus::prelude::*;
use maximus_api::models::room::Room;
use maximus_api::rooms::list::UserRoomList;
use crate::custom_components::layouts::main_layout::MainLayout;
use crate::custom_components::layouts::server_layout::ServerLayout;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::server::Server;

mod components;
mod pages;
mod custom_components;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/login")]
    Login,

    #[layout(MainLayout)]
    #[route("/")]
    Home,

    #[nest("/server/:server_id")]
    #[layout(ServerLayout)]
    #[route("/")]
    Server {
        server_id: String
    },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const DIOXUS_COMPONENTS: Asset = asset!("/assets/dx-components-theme.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

static ROOMS: GlobalSignal<Option<UserRoomList>> = Signal::global(|| None);

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: DIOXUS_COMPONENTS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}