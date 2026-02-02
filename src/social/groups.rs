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
use crate::database::models::{Group, GroupMember, GroupRole, CreateGroupRequest, ChatListItem, GroupFilter};
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
