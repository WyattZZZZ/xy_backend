use axum::{
    extract::{State, Path, Query},
    Json,
    response::IntoResponse,
    http::{StatusCode, HeaderMap},
};
use chrono::Utc;
use std::sync::Arc;
use crate::database::Database;
use crate::database::models::{Follow, FollowFilter};
use crate::verify::auth::get_user_id_from_token;

pub async fn get_follows(
    Query(filter): Query<FollowFilter>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let db_follows = db.follows.lock().unwrap();
    let db_users = db.users.lock().unwrap();
    
    let results: Vec<crate::database::models::FollowUserItem> = db_follows.iter()
        .filter(|f| {
            let match_follower = filter.follower_id.as_ref().map_or(true, |id| &f.follower_id == id);
            let match_following = filter.following_id.as_ref().map_or(true, |id| &f.following_id == id);
            match_follower && match_following
        })
        .map(|f| {
            // If we are looking for followers (following_id is fixed), we want the follower's info
            // If we are looking for following (follower_id is fixed), we want the following target's info
            // If both or neither are provided, we default to the "other" side based on common usage
            let target_id = if filter.follower_id.is_some() {
                &f.following_id
            } else {
                &f.follower_id
            };

            if let Some(user) = db_users.get(target_id) {
                crate::database::models::FollowUserItem {
                    id: user.id.clone(),
                    name: user.name.clone(),
                    avatar: user.avatar.clone(),
                }
            } else {
                crate::database::models::FollowUserItem {
                    id: target_id.clone(),
                    name: "Unknown User".to_string(),
                    avatar: "https://picsum.photos/200".to_string(),
                }
            }
        })
        .collect();
        
    Json(results).into_response()
}

pub async fn follow_user(
    Path(id): Path<String>,
    headers: HeaderMap,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"status": "error", "message": "Unauthorized"}))).into_response(),
    };
    
    if authenticated_id == id {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"status": "error", "message": "Cannot follow yourself"}))).into_response();
    }

    let mut follow_added = false;
    {
        let mut db_follows = db.follows.lock().unwrap();
        if !db_follows.iter().any(|f| f.follower_id == authenticated_id && f.following_id == id) {
            db_follows.push(Follow {
                follower_id: authenticated_id.clone(),
                following_id: id.clone(),
                created_at: Utc::now(),
            });
            follow_added = true;
        }
    }

    if follow_added {
        let mut db_users = db.users.lock().unwrap();
        // Update follower's following count
        if let Some(user) = db_users.get_mut(&authenticated_id) {
            user.following += 1;
        }
        // Update following's fans count
        if let Some(target) = db_users.get_mut(&id) {
            target.fans += 1;
        }
        drop(db_users);
        db.save_all();
    }

    Json(serde_json::json!({"status": "ok"})).into_response()
}

pub async fn unfollow_user(
    Path(id): Path<String>,
    headers: HeaderMap,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"status": "error", "message": "Unauthorized"}))).into_response(),
    };
    
    let mut follow_removed = false;
    {
        let mut db_follows = db.follows.lock().unwrap();
        let initial_len = db_follows.len();
        db_follows.retain(|f| !(f.follower_id == authenticated_id && f.following_id == id));
        if db_follows.len() < initial_len {
            follow_removed = true;
        }
    }

    if follow_removed {
        let mut db_users = db.users.lock().unwrap();
        // Update follower's following count
        if let Some(user) = db_users.get_mut(&authenticated_id) {
            if user.following > 0 { user.following -= 1; }
        }
        // Update following's fans count
        if let Some(target) = db_users.get_mut(&id) {
            if target.fans > 0 { target.fans -= 1; }
        }
        drop(db_users);
        db.save_all();
    }

    Json(serde_json::json!({"status": "ok"})).into_response()
}
