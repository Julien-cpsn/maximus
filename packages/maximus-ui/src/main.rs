use dioxus::prelude::*;
use crate::custom_components::nav_bar::Navbar;
use crate::pages::home::Home;
use crate::pages::login::Login;

mod components;
mod pages;
mod custom_components;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/login")]
    Login,
    #[layout(Navbar)]
    #[route("/")]
    Home,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const DIOXUS_COMPONENTS: Asset = asset!("/assets/dx-components-theme.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

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