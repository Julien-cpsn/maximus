use dioxus::prelude::*;

#[component]
pub fn ChevronIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "sidebar-icon sidebar-chevron",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "m9 18 6-6-6-6" }
        }
    }
}