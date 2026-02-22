use dioxus::prelude::*;
use crate::components::separator::Separator;
use crate::components::sidebar::{Sidebar, SidebarCollapsible, SidebarContent, SidebarGroup, SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu, SidebarProvider, SidebarRail, SidebarSide, SidebarTrigger, SidebarVariant};
use crate::custom_components::utils::spinner::spinner::{Spinner, SpinnerSize};
use crate::{Route, ROOMS};

#[component]
pub fn ServerLayout(server_id: String) -> Element {
    let nav = navigator();

    let rooms = &*ROOMS.read();

    if rooms.is_none() {
        return rsx! {
            Spinner {
                size: SpinnerSize::Medium
            }
        }
    }

    let rooms = rooms.as_ref().unwrap();
    let room = rooms.servers.iter().find(|(room_id, _)| *room_id == &server_id).clone();

    if room.is_none() {
        nav.push(Route::Home);
        return rsx! {}
    }

    let (_, room) = room.unwrap();

    rsx! {
        SidebarProvider {
            default_open: true,
            Sidebar {
                variant: SidebarVariant::Inset,
                collapsible: SidebarCollapsible::Icon,
                side: SidebarSide::Left,
                SidebarHeader {
                    "{room.display_name}"
                },
                SidebarContent {
                    for subroom in &room.children {
                        SidebarGroup {
                            SidebarGroupLabel { "{subroom.display_name}" }
                            SidebarMenu {

                            }
                        }
                    }
                }
                SidebarRail {}
            },
            SidebarInset {
                header { style: "display:flex; align-items:center; justify-content:space-between; height:3.5rem; flex-shrink:0; padding:0 1rem; border-bottom:1px solid var(--sidebar-border); background:var(--primary-color-1);",
                    div { style: "display: flex; align-items: center; gap: 0.75rem;",
                        SidebarTrigger {}
                        Separator { height: "1rem", horizontal: false }
                        span { "Sidebar Setting" }
                    }
                }
                div { style: "display:flex; flex:1; flex-direction:column; gap:1.5rem; padding:1.5rem; min-height:0; overflow-y:auto; overflow-x:hidden;",
                    p { "test" }
                }
            }
        }
    }
}