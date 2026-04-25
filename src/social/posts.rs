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
use crate::database::models::{CreatePostRequest, PostFilter, post_from_row};
use crate::verify::auth::get_user_id_from_token;

pub async fn create_post(
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Json(req): Json<CreatePostRequest>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let now = Utc::now();
    let post_id = Uuid::new_v4();

    let result = sqlx::query(
        "INSERT INTO posts (id, author_id, title, content, category, location, tags, media, likes_count, comments_count, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 0, 0, $9, $9)"
    )
    .bind(post_id)
    .bind(&authenticated_id)
    .bind(&req.title)
    .bind(&req.content)
    .bind(&req.category)
    .bind(&req.location)
    .bind(&req.tags)
    .bind(&req.media)
    .bind(now)
    .execute(&db.pool)
    .await;

    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    let _ = sqlx::query("UPDATE users SET posts = posts + 1 WHERE id = $1")
        .bind(&authenticated_id)
        .execute(&db.pool)
        .await;

    let row = sqlx::query("SELECT * FROM posts WHERE id = $1").bind(post_id).fetch_one(&db.pool).await;
    match row {
        Ok(r) => (StatusCode::CREATED, Json(post_from_row(&r))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn update_post(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Json(req): Json<CreatePostRequest>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let author: Option<String> = sqlx::query_scalar("SELECT author_id FROM posts WHERE id = $1")
        .bind(id).fetch_optional(&db.pool).await.unwrap_or(None);

    match author {
        None => return (StatusCode::NOT_FOUND, "Post not found").into_response(),
        Some(aid) if aid != authenticated_id => return (StatusCode::FORBIDDEN, "Not your post").into_response(),
        _ => {}
    }

    let _ = sqlx::query(
        "UPDATE posts SET title=$1, content=$2, category=$3, location=$4, tags=$5, media=$6, updated_at=$7 WHERE id=$8"
    )
    .bind(&req.title).bind(&req.content).bind(&req.category).bind(&req.location)
    .bind(&req.tags).bind(&req.media).bind(Utc::now()).bind(id)
    .execute(&db.pool).await;

    let row = sqlx::query("SELECT * FROM posts WHERE id = $1").bind(id).fetch_one(&db.pool).await;
    match row {
        Ok(r) => (StatusCode::OK, Json(post_from_row(&r))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn get_posts(
    State(db): State<Arc<Database>>,
    Query(filter): Query<PostFilter>,
) -> impl IntoResponse {
    let mut sql = "SELECT * FROM posts WHERE 1=1".to_string();

    if let Some(ref cat) = filter.category {
        let cat_esc = cat.replace('\'', "''");
        sql.push_str(&format!(" AND category = '{}'", cat_esc));
    }
    if let Some(ref aid) = filter.author_id {
        let aid_esc = aid.replace('\'', "''");
        sql.push_str(&format!(" AND author_id = '{}'", aid_esc));
    }
    if let Some(ref q) = filter.query {
        let q_esc = q.replace('\'', "''");
        sql.push_str(&format!(" AND LOWER(content) LIKE LOWER('%{}%')", q_esc));
    }
    sql.push_str(" ORDER BY created_at DESC");

    let rows = sqlx::query(&sql).fetch_all(&db.pool).await;
    match rows {
        Ok(rows) => Json(rows.iter().map(|r| post_from_row(r)).collect::<Vec<_>>()).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn get_post(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let row = sqlx::query("SELECT * FROM posts WHERE id = $1")
        .bind(id).fetch_optional(&db.pool).await;

    match row {
        Ok(Some(r)) => Json(post_from_row(&r)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Post not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn delete_post(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let author: Option<String> = sqlx::query_scalar("SELECT author_id FROM posts WHERE id = $1")
        .bind(id).fetch_optional(&db.pool).await.unwrap_or(None);

    match author {
        None => return (StatusCode::NOT_FOUND, "Post not found").into_response(),
        Some(aid) if aid != authenticated_id => return (StatusCode::FORBIDDEN, "Not your post").into_response(),
        _ => {}
    }

    let _ = sqlx::query("DELETE FROM posts WHERE id = $1").bind(id).execute(&db.pool).await;
    StatusCode::NO_CONTENT.into_response()
}

pub async fn get_post_comments(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    _headers: HeaderMap,
) -> impl IntoResponse {
    let comment_rows = sqlx::query(
        "SELECT pc.*, u.name as user_name, u.avatar as user_avatar
         FROM post_comments pc
         LEFT JOIN users u ON u.id = pc.user_id
         WHERE pc.post_id = $1
         ORDER BY pc.created_at ASC"
    )
    .bind(id)
    .fetch_all(&db.pool)
    .await
    .unwrap_or_default();

    let reply_rows = sqlx::query(
        "SELECT cr.*, u.name as user_name, u.avatar as user_avatar
         FROM comment_replies cr
         LEFT JOIN users u ON u.id = cr.user_id
         WHERE cr.post_id = $1
         ORDER BY cr.created_at ASC"
    )
    .bind(id)
    .fetch_all(&db.pool)
    .await
    .unwrap_or_default();

    let comments: Vec<crate::database::models::CommentResponse> = comment_rows.iter().map(|cr| {
        let comment_id: Uuid = sqlx::Row::get(cr, "id");
        let replies: Vec<crate::database::models::ReplyResponse> = reply_rows.iter()
            .filter(|rr| {
                let rid: Uuid = sqlx::Row::get(*rr, "comment_id");
                rid == comment_id
            })
            .map(|rr| crate::database::models::ReplyResponse {
                id: sqlx::Row::get(rr, "id"),
                comment_id: sqlx::Row::get(rr, "comment_id"),
                user_id: sqlx::Row::get(rr, "user_id"),
                user_name: sqlx::Row::try_get(rr, "user_name").ok().unwrap_or_else(|| "Unknown".to_string()),
                user_avatar: sqlx::Row::try_get(rr, "user_avatar").ok().unwrap_or_default(),
                content: sqlx::Row::get(rr, "content"),
                created_at: sqlx::Row::get(rr, "created_at"),
            })
            .collect();

        crate::database::models::CommentResponse {
            id: comment_id,
            post_id: sqlx::Row::get(cr, "post_id"),
            user_id: sqlx::Row::get(cr, "user_id"),
            user_name: sqlx::Row::try_get(cr, "user_name").ok().unwrap_or_else(|| "Unknown".to_string()),
            user_avatar: sqlx::Row::try_get(cr, "user_avatar").ok().unwrap_or_default(),
            content: sqlx::Row::get(cr, "content"),
            likes_count: 0,
            is_liked: false,
            replies,
            created_at: sqlx::Row::get(cr, "created_at"),
        }
    }).collect();

    Json(comments).into_response()
}

pub async fn add_comment(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Json(req): Json<crate::database::models::PostCommentRequest>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1)")
        .bind(id).fetch_one(&db.pool).await.unwrap_or(false);
    if !exists {
        return (StatusCode::NOT_FOUND, "Post not found").into_response();
    }

    let comment_id = Uuid::new_v4();
    let now = Utc::now();

    let result = sqlx::query(
        "INSERT INTO post_comments (id, post_id, user_id, content, created_at) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(comment_id).bind(id).bind(&authenticated_id).bind(&req.content).bind(now)
    .execute(&db.pool).await;

    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    let _ = sqlx::query("UPDATE posts SET comments_count = comments_count + 1 WHERE id = $1")
        .bind(id).execute(&db.pool).await;

    let comment = crate::database::models::PostComment {
        id: comment_id,
        post_id: id,
        user_id: authenticated_id,
        content: req.content,
        created_at: now,
    };
    (StatusCode::CREATED, Json(comment)).into_response()
}

pub async fn add_reply(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Json(req): Json<crate::database::models::PostReplyRequest>,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let reply_id = Uuid::new_v4();
    let now = Utc::now();

    let result = sqlx::query(
        "INSERT INTO comment_replies (id, comment_id, post_id, user_id, content, created_at) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(reply_id).bind(req.comment_id).bind(id).bind(&authenticated_id).bind(&req.content).bind(now)
    .execute(&db.pool).await;

    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    let _ = sqlx::query("UPDATE posts SET comments_count = comments_count + 1 WHERE id = $1")
        .bind(id).execute(&db.pool).await;

    let reply = crate::database::models::PostCommentReply {
        id: reply_id,
        comment_id: req.comment_id,
        post_id: id,
        user_id: authenticated_id,
        content: req.content,
        created_at: now,
    };
    (StatusCode::CREATED, Json(reply)).into_response()
}

pub async fn get_replies(
    Path(comment_id): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let rows = sqlx::query("SELECT * FROM comment_replies WHERE comment_id = $1 ORDER BY created_at ASC")
        .bind(comment_id)
        .fetch_all(&db.pool)
        .await
        .unwrap_or_default();

    let replies: Vec<crate::database::models::PostCommentReply> = rows.iter().map(|r| {
        crate::database::models::PostCommentReply {
            id: sqlx::Row::get(r, "id"),
            comment_id: sqlx::Row::get(r, "comment_id"),
            post_id: sqlx::Row::get(r, "post_id"),
            user_id: sqlx::Row::get(r, "user_id"),
            content: sqlx::Row::get(r, "content"),
            created_at: sqlx::Row::get(r, "created_at"),
        }
    }).collect();

    Json(replies).into_response()
}

pub async fn delete_comment(
    Path((post_id, comment_id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let owner: Option<String> = sqlx::query_scalar("SELECT user_id FROM post_comments WHERE id = $1")
        .bind(comment_id).fetch_optional(&db.pool).await.unwrap_or(None);

    match owner {
        None => return (StatusCode::NOT_FOUND, "Comment not found").into_response(),
        Some(uid) if uid != authenticated_id => return (StatusCode::FORBIDDEN, "Not your comment").into_response(),
        _ => {}
    }

    let reply_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM comment_replies WHERE comment_id = $1")
        .bind(comment_id).fetch_one(&db.pool).await.unwrap_or(0);

    // Cascade deletes replies via FK
    let _ = sqlx::query("DELETE FROM post_comments WHERE id = $1").bind(comment_id).execute(&db.pool).await;

    let _ = sqlx::query(
        "UPDATE posts SET comments_count = GREATEST(comments_count - $1, 0) WHERE id = $2"
    )
    .bind(1 + reply_count as i32).bind(post_id).execute(&db.pool).await;

    StatusCode::NO_CONTENT.into_response()
}

pub async fn check_post_favorite(
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Path(post_id): Path<Uuid>,
) -> impl IntoResponse {
    let uid = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let is_favorited: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM post_favorites WHERE post_id = $1 AND user_id = $2)"
    )
    .bind(post_id).bind(&uid)
    .fetch_one(&db.pool).await.unwrap_or(false);

    (StatusCode::OK, Json(crate::social::products::FavoriteStatusResponse { is_favorited })).into_response()
}

pub async fn favorite_post(
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Path(post_id): Path<Uuid>,
) -> impl IntoResponse {
    let uid = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let _ = sqlx::query(
        "INSERT INTO post_favorites (post_id, user_id, created_at) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
    )
    .bind(post_id).bind(&uid).bind(Utc::now())
    .execute(&db.pool).await;

    (StatusCode::OK, "Favorited").into_response()
}

pub async fn unfavorite_post(
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
    Path(post_id): Path<Uuid>,
) -> impl IntoResponse {
    let uid = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let _ = sqlx::query("DELETE FROM post_favorites WHERE post_id = $1 AND user_id = $2")
        .bind(post_id).bind(&uid).execute(&db.pool).await;

    (StatusCode::OK, "Unfavorited").into_response()
}
