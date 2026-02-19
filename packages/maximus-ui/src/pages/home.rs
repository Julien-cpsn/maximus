use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-row",
            div {
                class: "w-140 h-screen p-4 bg-neutral-950",
                "Servers"
            },
            div {
                class: "w-full h-screen p-4 bg-neutral-900",
                "Messages"
            }
        }
    }
}