use dioxus::prelude::*;
use dioxus_primitives::ContentSide;
use crate::components::button::{Button, ButtonVariant};
use crate::components::tooltip::{Tooltip, TooltipContent, TooltipTrigger};

#[component]
pub fn CopyButton(text_to_copy: String) -> Element {
    rsx! {
        Tooltip {
            TooltipTrigger {
                Button {
                    class: "p-2",
                    variant: ButtonVariant::Outline,
                    "onclick": "navigator.clipboard.writeText(\"{text_to_copy}\");",
                    img {
                        width: "20px",
                        src: asset!("/assets/images/clipboard-outline.svg")
                    }
                }
            }
            TooltipContent {
                side: ContentSide::Top,
                p { "Copy to clipboard" }
            }
        }
    }
}