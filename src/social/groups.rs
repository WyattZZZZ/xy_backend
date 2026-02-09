use serde::Deserialize;
use axum::{
    extract::{State, Path, Query},
    Json,
    response::IntoResponse,
    http::{StatusCode, HeaderMap},
};
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;
use crate::database::Database;
use crate::database::models::{Group, GroupMember, GroupRole, CreateGroupRequest, ChatListItem, GroupFilter, GroupMessage, GroupMemberAction};
use crate::verify::auth::get_user_id_from_token;

pub async fn create_group(
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Json(req): Json<CreateGroupRequest>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };
    let now = Utc::now();
    let group_id = Uuid::new_v4();

    let group = Group {
        id: group_id,
        creator_id: authenticated_id.clone(),
        avatar: req.avatar.unwrap_or_else(|| "https://picsum.photos/seed/group/200/200".to_string()),
        max_member: 100,
        member_num: (req.participants.len() + 1) as i32,
        name: req.name,
        created_at: now,
    };

    let mut members = Vec::new();
    // Add creator
    members.push(GroupMember {
        group_id,
        user_id: authenticated_id,
        join_time: now,
        role: GroupRole::Admin,
        mute_until: None,
        is_muted: false,
    });

    // Add other participants
    for uid in req.participants {
        members.push(GroupMember {
            group_id,
            user_id: uid,
            join_time: now,
            role: GroupRole::Normal,
            mute_until: None,
            is_muted: false,
        });
    }

    {
        let mut db_groups = db.groups.lock().unwrap();
        db_groups.insert(group_id, group.clone());
        let mut db_members = db.group_members.lock().unwrap();
        db_members.extend(members);
    }
    db.save_all();

    (StatusCode::CREATED, Json(group)).into_response()
}

pub async fn get_groups(
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Query(filter): Query<GroupFilter>,
) -> impl IntoResponse {
    let user_id = filter.user_id.or_else(|| get_user_id_from_token(&headers)).unwrap_or_else(|| "test_user".to_string());
    let db_members = db.group_members.lock().unwrap();
    let group_ids: Vec<Uuid> = db_members.iter()
        .filter(|m| m.user_id == user_id)
        .map(|m| m.group_id)
        .collect();
    drop(db_members);

    let db_groups = db.groups.lock().unwrap();
    let my_groups: Vec<ChatListItem> = group_ids.iter()
        .filter_map(|gid| {
            db_groups.get(gid).map(|g| {
                ChatListItem {
                    id: g.id,
                    name: g.name.clone(),
                    avatar: g.avatar.clone(),
                    last_msg: "Group Chat".to_string(), // Placeholder
                    time: g.created_at.format("%H:%M").to_string(),
                    unread: false,
                    is_group: true,
                    other_user_id: None,
                }
            })
        })
        .collect();

    Json(my_groups).into_response()
}

pub async fn get_group(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let db_groups = db.groups.lock().unwrap();
    match db_groups.get(&id) {
        Some(g) => Json(Some(g.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, "Group not found").into_response(),
    }
}

pub async fn get_group_members(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let db_members = db.group_members.lock().unwrap();
    let members: Vec<GroupMember> = db_members.iter()
        .filter(|m| m.group_id == id)
        .cloned()
        .collect();
    Json(members).into_response()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupMessageRequest {
    pub content: String,
    pub msg_type: crate::database::models::MessageType,
    pub reply_id: Option<Uuid>,
}

pub async fn post_group_message(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Json(req): Json<CreateGroupMessageRequest>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    // Verify membership
    let is_member = {
        let members = db.group_members.lock().unwrap();
        members.iter().any(|m| m.group_id == id && m.user_id == authenticated_id)
    };

    if !is_member {
        return (StatusCode::FORBIDDEN, "Not a member of this group").into_response();
    }

    let msg = GroupMessage {
        id: Uuid::new_v4(),
        group_id: id,
        sender_id: authenticated_id,
        reply_id: req.reply_id,
        content: req.content,
        msg_type: req.msg_type,
        created_at: Utc::now(),
    };

    {
        let mut group_msgs = db.group_messages.lock().unwrap();
        group_msgs.push(msg.clone());
    }
    db.save_all();

    (StatusCode::CREATED, Json(msg)).into_response()
}

pub async fn get_group_messages(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    // Verify membership
    let is_member = {
        let members = db.group_members.lock().unwrap();
        members.iter().any(|m| m.group_id == id && m.user_id == authenticated_id)
    };

    if !is_member {
        return (StatusCode::FORBIDDEN, "Not a member of this group").into_response();
    }

    let msgs = {
        let group_msgs = db.group_messages.lock().unwrap();
        group_msgs.iter()
            .filter(|m| m.group_id == id)
            .cloned()
            .collect::<Vec<_>>()
    };

    Json(msgs).into_response()
}

pub async fn handle_group_action(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Json(action): Json<GroupMemberAction>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let mut members = db.group_members.lock().unwrap();
    
    // Check requester role
    let requester_role = members.iter()
        .find(|m| m.group_id == id && m.user_id == authenticated_id)
        .map(|m| m.role);

    if requester_role.is_none() {
        return (StatusCode::FORBIDDEN, "Not a member").into_response();
    }
    let is_admin = requester_role == Some(GroupRole::Admin);

    if action.leave {
        // Leave or Kick
        if action.target_id == authenticated_id {
            // Self leave
            members.retain(|m| !(m.group_id == id && m.user_id == authenticated_id));
        } else {
            // Kick
            if !is_admin {
                return (StatusCode::FORBIDDEN, "Only admin can remove members").into_response();
            }
            members.retain(|m| !(m.group_id == id && m.user_id == action.target_id));
        }
    } else {
        // Invite
        // Check if already member
        if members.iter().any(|m| m.group_id == id && m.user_id == action.target_id) {
            return (StatusCode::CONFLICT, "User already in group").into_response();
        }
        
        members.push(GroupMember {
            group_id: id,
            user_id: action.target_id.clone(),
            join_time: Utc::now(),
            role: GroupRole::Normal,
            mute_until: None,
            is_muted: false,
        });
    }

    // Update member count in group
    let count = members.iter().filter(|m| m.group_id == id).count() as i32;
    drop(members); // unlock before locking groups

    {
        let mut groups = db.groups.lock().unwrap();
        if let Some(g) = groups.get_mut(&id) {
            g.member_num = count;
        }
    }
    db.save_all();

    (StatusCode::OK, "Action successful").into_response()
}

pub async fn delete_group(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    // Verify admin
    let is_admin = {
        let members = db.group_members.lock().unwrap();
        members.iter().any(|m| m.group_id == id && m.user_id == authenticated_id && m.role == GroupRole::Admin)
    };

    if !is_admin {
        return (StatusCode::FORBIDDEN, "Only admin can delete group").into_response();
    }

    // Delete group
    {
        let mut groups = db.groups.lock().unwrap();
        groups.remove(&id);
    }
    
    // Delete members
    {
        let mut members = db.group_members.lock().unwrap();
        members.retain(|m| m.group_id != id);
    }

    // Delete messages
    {
        let mut msgs = db.group_messages.lock().unwrap();
        msgs.retain(|m| m.group_id != id);
    }

    db.save_all();

    (StatusCode::OK, "Group disbanded").into_response()
}

pub async fn update_group_info(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Json(req): Json<crate::database::models::UpdateGroupRequest>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    // Check if user is admin
    let is_admin = {
        let members = db.group_members.lock().unwrap();
        members.iter().any(|m| m.group_id == id && m.user_id == authenticated_id && m.role == GroupRole::Admin)
    };

    if !is_admin {
        return (StatusCode::FORBIDDEN, "Only admins can update group info").into_response();
    }

    let result = {
        let mut groups = db.groups.lock().unwrap();
        if let Some(group) = groups.get_mut(&id) {
            if let Some(name) = req.name {
                group.name = name;
            }
            if let Some(avatar) = req.avatar {
                group.avatar = avatar;
            }
            Some(group.clone())
        } else {
            None
        }
    };

    if let Some(group) = result {
        db.save_all();
        return (StatusCode::OK, Json(group)).into_response();
    }

    (StatusCode::NOT_FOUND, "Group not found").into_response()
}
