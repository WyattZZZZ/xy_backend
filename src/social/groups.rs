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
use crate::database::models::{
    ChatListItem, GroupFilter, GroupMemberAction, UpdateGroupRequest,
    CreateGroupRequest, group_from_row, group_member_from_row, group_message_from_row,
    MessageType, message_type_str,
};
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
    let avatar = req.avatar.unwrap_or_else(|| "https://picsum.photos/seed/group/200/200".to_string());
    let member_num = (req.participants.len() + 1) as i32;

    let result = sqlx::query(
        "INSERT INTO groups (id, creator_id, avatar, max_member, member_num, name, created_at)
         VALUES ($1, $2, $3, 100, $4, $5, $6)"
    )
    .bind(group_id)
    .bind(&authenticated_id)
    .bind(&avatar)
    .bind(member_num)
    .bind(&req.name)
    .bind(now)
    .execute(&db.pool)
    .await;

    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    let _ = sqlx::query(
        "INSERT INTO group_members (group_id, user_id, join_time, role, is_muted) VALUES ($1, $2, $3, 'ADMIN', FALSE)"
    )
    .bind(group_id)
    .bind(&authenticated_id)
    .bind(now)
    .execute(&db.pool)
    .await;

    for uid in &req.participants {
        let _ = sqlx::query(
            "INSERT INTO group_members (group_id, user_id, join_time, role, is_muted) VALUES ($1, $2, $3, 'NORMAL', FALSE)"
        )
        .bind(group_id)
        .bind(uid)
        .bind(now)
        .execute(&db.pool)
        .await;
    }

    let row = sqlx::query("SELECT * FROM groups WHERE id = $1").bind(group_id).fetch_one(&db.pool).await;
    match row {
        Ok(r) => (StatusCode::CREATED, Json(group_from_row(&r))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn get_groups(
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Query(filter): Query<GroupFilter>,
) -> impl IntoResponse {
    let user_id = filter.user_id
        .or_else(|| get_user_id_from_token(&headers))
        .unwrap_or_else(|| "test_user".to_string());

    let rows = sqlx::query(
        "SELECT g.* FROM groups g
         JOIN group_members gm ON g.id = gm.group_id
         WHERE gm.user_id = $1"
    )
    .bind(&user_id)
    .fetch_all(&db.pool)
    .await
    .unwrap_or_default();

    let items: Vec<ChatListItem> = rows.iter().map(|r| {
        let g = group_from_row(r);
        ChatListItem {
            id: g.id,
            name: g.name,
            avatar: g.avatar,
            last_msg: "Group Chat".to_string(),
            time: g.created_at.format("%H:%M").to_string(),
            unread: false,
            is_group: true,
            other_user_id: None,
        }
    }).collect();

    Json(items).into_response()
}

pub async fn get_group(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let row = sqlx::query("SELECT * FROM groups WHERE id = $1")
        .bind(id)
        .fetch_optional(&db.pool)
        .await;

    match row {
        Ok(Some(r)) => Json(Some(group_from_row(&r))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Group not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn get_group_members(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let rows = sqlx::query("SELECT * FROM group_members WHERE group_id = $1")
        .bind(id)
        .fetch_all(&db.pool)
        .await
        .unwrap_or_default();

    let members: Vec<_> = rows.iter().map(|r| group_member_from_row(r)).collect();
    Json(members).into_response()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupMessageRequest {
    pub content: String,
    pub msg_type: MessageType,
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

    let is_member: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM group_members WHERE group_id = $1 AND user_id = $2)"
    )
    .bind(id)
    .bind(&authenticated_id)
    .fetch_one(&db.pool)
    .await
    .unwrap_or(false);

    if !is_member {
        return (StatusCode::FORBIDDEN, "Not a member of this group").into_response();
    }

    let msg_id = Uuid::new_v4();
    let now = Utc::now();
    let msg_type_s = message_type_str(&req.msg_type);

    let result = sqlx::query(
        "INSERT INTO group_messages (id, group_id, sender_id, reply_id, content, msg_type, created_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(msg_id)
    .bind(id)
    .bind(&authenticated_id)
    .bind(req.reply_id)
    .bind(&req.content)
    .bind(msg_type_s)
    .bind(now)
    .execute(&db.pool)
    .await;

    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    let row = sqlx::query("SELECT * FROM group_messages WHERE id = $1").bind(msg_id).fetch_one(&db.pool).await;
    match row {
        Ok(r) => (StatusCode::CREATED, Json(group_message_from_row(&r))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
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

    let is_member: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM group_members WHERE group_id = $1 AND user_id = $2)"
    )
    .bind(id)
    .bind(&authenticated_id)
    .fetch_one(&db.pool)
    .await
    .unwrap_or(false);

    if !is_member {
        return (StatusCode::FORBIDDEN, "Not a member of this group").into_response();
    }

    let rows = sqlx::query("SELECT * FROM group_messages WHERE group_id = $1 ORDER BY created_at ASC")
        .bind(id)
        .fetch_all(&db.pool)
        .await
        .unwrap_or_default();

    let msgs: Vec<_> = rows.iter().map(|r| group_message_from_row(r)).collect();
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

    let requester_role: Option<String> = sqlx::query_scalar(
        "SELECT role FROM group_members WHERE group_id = $1 AND user_id = $2"
    )
    .bind(id)
    .bind(&authenticated_id)
    .fetch_optional(&db.pool)
    .await
    .unwrap_or(None);

    if requester_role.is_none() {
        return (StatusCode::FORBIDDEN, "Not a member").into_response();
    }
    let is_admin = requester_role.as_deref() == Some("ADMIN");

    if action.leave {
        if action.target_id == authenticated_id {
            let _ = sqlx::query("DELETE FROM group_members WHERE group_id = $1 AND user_id = $2")
                .bind(id).bind(&authenticated_id).execute(&db.pool).await;
        } else {
            if !is_admin {
                return (StatusCode::FORBIDDEN, "Only admin can remove members").into_response();
            }
            let _ = sqlx::query("DELETE FROM group_members WHERE group_id = $1 AND user_id = $2")
                .bind(id).bind(&action.target_id).execute(&db.pool).await;
        }
    } else {
        let already: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM group_members WHERE group_id = $1 AND user_id = $2)"
        )
        .bind(id).bind(&action.target_id)
        .fetch_one(&db.pool).await.unwrap_or(false);

        if already {
            return (StatusCode::CONFLICT, "User already in group").into_response();
        }
        let _ = sqlx::query(
            "INSERT INTO group_members (group_id, user_id, join_time, role, is_muted) VALUES ($1, $2, $3, 'NORMAL', FALSE)"
        )
        .bind(id).bind(&action.target_id).bind(Utc::now())
        .execute(&db.pool).await;
    }

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM group_members WHERE group_id = $1")
        .bind(id).fetch_one(&db.pool).await.unwrap_or(0);

    let _ = sqlx::query("UPDATE groups SET member_num = $1 WHERE id = $2")
        .bind(count as i32).bind(id).execute(&db.pool).await;

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

    let is_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM group_members WHERE group_id = $1 AND user_id = $2 AND role = 'ADMIN')"
    )
    .bind(id).bind(&authenticated_id)
    .fetch_one(&db.pool).await.unwrap_or(false);

    if !is_admin {
        return (StatusCode::FORBIDDEN, "Only admin can delete group").into_response();
    }

    // Cascade deletes group_members and group_messages via FK
    let _ = sqlx::query("DELETE FROM groups WHERE id = $1").bind(id).execute(&db.pool).await;
    (StatusCode::OK, "Group disbanded").into_response()
}

pub async fn update_group_info(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Json(req): Json<UpdateGroupRequest>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let is_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM group_members WHERE group_id = $1 AND user_id = $2 AND role = 'ADMIN')"
    )
    .bind(id).bind(&authenticated_id)
    .fetch_one(&db.pool).await.unwrap_or(false);

    if !is_admin {
        return (StatusCode::FORBIDDEN, "Only admins can update group info").into_response();
    }

    let result = sqlx::query(
        "UPDATE groups SET
            name = COALESCE($1, name),
            avatar = COALESCE($2, avatar)
         WHERE id = $3"
    )
    .bind(req.name)
    .bind(req.avatar)
    .bind(id)
    .execute(&db.pool)
    .await;

    match result {
        Ok(r) if r.rows_affected() == 0 => (StatusCode::NOT_FOUND, "Group not found").into_response(),
        Ok(_) => {
            let row = sqlx::query("SELECT * FROM groups WHERE id = $1").bind(id).fetch_one(&db.pool).await;
            match row {
                Ok(r) => (StatusCode::OK, Json(group_from_row(&r))).into_response(),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
            }
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}
