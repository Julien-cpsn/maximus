use dioxus::prelude::*;
use dioxus_primitives::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use maximus_shared_models::room::RoomType;
use crate::dioxus_components::sidebar::{Sidebar, SidebarCollapsible, SidebarContent, SidebarHeader, SidebarInset, SidebarMenuBadge, SidebarMenuButton, SidebarMenuItem, SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubItem, SidebarProvider, SidebarRail, SidebarSide, SidebarTrigger, SidebarVariant};
use crate::dioxus_components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};
use crate::custom_components::utils::chevron_icon::ChevronIcon;
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
    let room = rooms.servers.iter().find(|(room_id, _)| *room_id == &server_id);

    if room.is_none() {
        nav.push(Route::Home);
        return rsx! {};
    }

    let (_, room) = room.clone().unwrap();

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
                        if subroom.children.is_empty() {
                            SidebarMenuItem {
                                key: "{subroom.room_id}",
                                SidebarMenuButton {
                                    RoomAvatarOrIcon {
                                        room_type: subroom.room_type,
                                        avatar: subroom.avatar.clone()
                                    },
                                    span { "{subroom.display_name}" }
                                }
                                RoomUnreadMessageCount {
                                    unread_message_count: subroom.unread_messages_count
                                }
                            }
                        }
                        else {
                            Collapsible {
                                default_open: true,
                                SidebarMenuItem {
                                    key: "{subroom.room_id}",
                                    CollapsibleTrigger {
                                        SidebarMenuButton {
                                            tooltip: rsx! { "{subroom.display_name}" },
                                            if let Some(avatar) = &subroom.avatar {
                                                Avatar {
                                                    size: AvatarImageSize::Small,
                                                    aria_label: "Room avatar",
                                                    AvatarImage {
                                                        src: "data:image/jpeg;base64,{avatar}",
                                                        alt: "Room avatar",
                                                    },
                                                    AvatarFallback { class: "avatar-fallback", "RA" }
                                                },
                                            },
                                            span { "{subroom.display_name}" },
                                            ChevronIcon {}
                                        }
                                    }
                                    CollapsibleContent {
                                        SidebarMenuSub {
                                            for channel in &subroom.children {
                                                SidebarMenuSubItem {
                                                    key: "{channel.room_id}",
                                                    SidebarMenuSubButton {
                                                        span { "{channel.display_name}" }
                                                        RoomUnreadMessageCount {
                                                            unread_message_count: channel.unread_messages_count
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            SidebarInset {
                header { style: "display:flex; align-items:center; justify-content:space-between; height:3.5rem; flex-shrink:0; padding:0 1rem; border-bottom:1px solid var(--sidebar-border); background:var(--primary-color-1);",
                    div { style: "display: flex; align-items: center; gap: 0.75rem;",
                        span { "Sidebar Setting" }
                    }
                }
                div { style: "display:flex; flex:1; flex-direction:column; gap:1.5rem; padding:1.5rem; min-height:0; overflow-y:auto; overflow-x:hidden;",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[component]
fn RoomAvatarOrIcon(room_type: RoomType, avatar: Option<String>) -> Element {
    if let Some(avatar) = avatar {
        rsx! {
            Avatar {
                size: AvatarImageSize::Small,
                aria_label: "Room avatar",
                AvatarImage {
                    src: "data:image/jpeg;base64,{avatar}",
                    alt: "Room avatar",
                },
                AvatarFallback { class: "avatar-fallback", "RA" }
            }
        }
    }
    else {
        match room_type {
            RoomType::Space => rsx! { },
            RoomType::Call => rsx! { span { "🔊" } },
            RoomType::Text => rsx! { span { "#" } }
        }
    }
}

#[component]
fn RoomUnreadMessageCount(unread_message_count: u64) -> Element {
    rsx! {
        if unread_message_count == 0 {

        }
        else if unread_message_count > 99 {
            SidebarMenuBadge { "+99" }
        }
        else {
            SidebarMenuBadge { "{unread_message_count}" }
        }
    }
}