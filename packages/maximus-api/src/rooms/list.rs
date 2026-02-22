use std::collections::HashMap;
use base64::engine::{general_purpose, Engine as _};
use dioxus::prelude::*;
use matrix_sdk::media::MediaFormat;
use matrix_sdk::room::ParentSpace;
use matrix_sdk::stream::StreamExt;
use serde::{Deserialize, Serialize};
use crate::models::room::RoomPreview;
use crate::user::login::{get_client};


#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct UserRoomList {
    pub servers: HashMap<String, RoomPreview>,
    pub dms: HashMap<String, RoomPreview>,
}

#[server]
pub async fn get_user_rooms() -> Result<UserRoomList> {
    let client = get_client()?;

    let matrix_rooms = client.joined_rooms();

    let mut dms: HashMap<String, RoomPreview> = HashMap::new();
    let mut servers: HashMap<String, RoomPreview> = HashMap::new();
    let mut parent_map: HashMap<String, Vec<String>> = HashMap::new();

    for matrix_room in matrix_rooms {
        let room_id = matrix_room.room_id().to_string();
        let display_name = matrix_room.display_name().await?.to_string();
        let avatar = matrix_room.avatar(MediaFormat::File).await?;

        // Collect parent IDs
        let parents: Vec<String> = matrix_room
            .parent_spaces()
            .await?
            .filter_map(|e| async move {
                if let Ok(parent_space) = e {
                    match parent_space {
                        ParentSpace::Reciprocal(room)
                        | ParentSpace::WithPowerlevel(room)
                        | ParentSpace::Illegitimate(room) => {
                            Some(room.room_id().to_string())
                        }
                        ParentSpace::Unverifiable(_) => None,
                    }
                } else {
                    None
                }
            })
            .collect()
            .await;

        let room = RoomPreview {
            room_id: room_id.clone(),
            display_name,
            state: matrix_room.state(),
            children: vec![],
            avatar: avatar.map(|data| general_purpose::STANDARD.encode(&data)),
        };

        if matrix_room.is_direct().await? {
            dms.insert(room_id, room);
        }
        else {
            servers.insert(room_id.clone(), room);

            for parent in parents {
                parent_map.entry(parent).or_default().push(room_id.clone());
            }
        }
    }

    for (parent_id, children_ids) in &parent_map {
        let children: Vec<RoomPreview> = children_ids
            .iter()
            .filter_map(|child_id| servers.get(child_id).cloned())
            .collect();

        if let Some(parent_room) = servers.get_mut(parent_id) {
            parent_room.children.extend(children);
        }
    }

    let mut child_ids = std::collections::HashSet::new();
    for children in parent_map.values() {
        for child in children {
            child_ids.insert(child.clone());
        }
    }

    let mut complete_servers = HashMap::new();

    for (room_id, room) in servers {
        if !child_ids.contains(&room_id) {
            complete_servers.insert(room_id, room);
        }
    }

    Ok(UserRoomList {
        servers: complete_servers,
        dms,
    })
}