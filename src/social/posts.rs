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
use crate::database::models::{Post, CreatePostRequest, PostFilter};
use crate::verify::auth::get_user_id_from_token;

// Return whether the current user has liked a given post
pub async fn get_post_like_status(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };
    let likes = db.post_likes.lock().unwrap();
    let is_liked = likes.iter().any(|l| l.post_id == id && l.user_id == authenticated_id);
    Json(serde_json::json!({ "isLiked": is_liked })).into_response()
}

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

    let post = Post {
        id: post_id,
        author_id: authenticated_id.clone(),
        title: req.title,
        content: req.content,
        category: req.category,
        location: req.location,
        tags: req.tags,
        media: req.media,
        likes_count: 0,
        comments_count: 0,
        created_at: now,
        updated_at: now,
    };

    {
        let mut posts = db.posts.lock().unwrap();
        posts.insert(post_id, post.clone());
    }
    
    // Increment user post count
    {
        let mut users = db.users.lock().unwrap();
        if let Some(user) = users.get_mut(&authenticated_id) {
            user.posts += 1;
        }
    }
    
    db.save_all();

    (StatusCode::CREATED, Json(post)).into_response()
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

    let mut posts = db.posts.lock().unwrap();
    if let Some(post) = posts.get_mut(&id) {
        if post.author_id != authenticated_id {
            return (StatusCode::FORBIDDEN, "Not your post").into_response();
        }
        
        post.title = req.title;
        post.content = req.content;
        post.category = req.category;
        post.location = req.location;
        post.tags = req.tags;
        post.media = req.media;
        post.updated_at = Utc::now();
        
        let updated_post = post.clone();
        drop(posts);
        db.save_all();
        (StatusCode::OK, Json(updated_post)).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Post not found").into_response()
    }
}

pub async fn get_posts(
    State(db): State<Arc<Database>>,
    Query(filter): Query<PostFilter>,
) -> impl IntoResponse {
    let posts_map = db.posts.lock().unwrap();
    let mut posts: Vec<Post> = posts_map.values()
        .filter(|p| {
            if let Some(cat) = &filter.category {
                if p.category != *cat { return false; }
            }
            if let Some(aid) = &filter.author_id {
                if p.author_id != *aid { return false; }
            }
            if let Some(q) = &filter.query {
                let q_lower = q.to_lowercase();
                if !p.content.to_lowercase().contains(&q_lower) {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    // Sort by created_at desc
    posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Json(posts).into_response()
}

pub async fn get_post(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let posts = db.posts.lock().unwrap();
    match posts.get(&id) {
        Some(p) => Json(Some(p.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, "Post not found").into_response(),
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

    let mut posts = db.posts.lock().unwrap();
    if let Some(post) = posts.get(&id) {
        if post.author_id != authenticated_id {
            return (StatusCode::FORBIDDEN, "Not your post").into_response();
        }
        posts.remove(&id);
        drop(posts);
        db.save_all();
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, "Post not found").into_response()
    }
}

pub async fn like_post(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    // Check if post exists
    {
        let posts = db.posts.lock().unwrap();
        if !posts.contains_key(&id) {
            return (StatusCode::NOT_FOUND, "Post not found").into_response();
        }
    }

    let mut likes = db.post_likes.lock().unwrap();
    if likes.iter().any(|l| l.post_id == id && l.user_id == authenticated_id) {
        return (StatusCode::BAD_REQUEST, "Already liked").into_response();
    }

    likes.push(crate::database::models::PostLike {
        post_id: id,
        user_id: authenticated_id,
        created_at: Utc::now(),
    });
    drop(likes);

    // Update post like count
    {
        let mut posts = db.posts.lock().unwrap();
        if let Some(post) = posts.get_mut(&id) {
            post.likes_count += 1;
        }
    }

    db.save_all();
    StatusCode::OK.into_response()
}

pub async fn unlike_post(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let mut likes = db.post_likes.lock().unwrap();
    let initial_len = likes.len();
    likes.retain(|l| !(l.post_id == id && l.user_id == authenticated_id));
    
    if likes.len() < initial_len {
        drop(likes);
        let mut posts = db.posts.lock().unwrap();
        if let Some(post) = posts.get_mut(&id) {
            post.likes_count = (post.likes_count - 1).max(0);
        }
        db.save_all();
        StatusCode::OK.into_response()
    } else {
        (StatusCode::NOT_FOUND, "Like not found").into_response()
    }
}

pub async fn get_post_comments(
    Path(id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let viewer_id = get_user_id_from_token(&headers);

    // Collect comment data first (release lock immediately)
    let comments_data: Vec<crate::database::models::PostComment> = {
        let comments = db.post_comments.lock().unwrap();
        comments.iter()
            .filter(|c| c.post_id == id)
            .cloned()
            .collect()
    };

    // Collect comment likes count per comment (release lock)
    let likes_data: Vec<crate::database::models::CommentLike> = {
        let clikes = db.comment_likes.lock().unwrap();
        clikes.iter()
            .filter(|l| comments_data.iter().any(|c| c.id == l.comment_id))
            .cloned()
            .collect()
    };

    let post_comments: Vec<crate::database::models::CommentResponse> = {
        let users = db.users.lock().unwrap();
        comments_data.iter().map(|c| {
            let (name, avatar) = if let Some(u) = users.get(&c.user_id) {
                (u.name.clone(), u.avatar.clone())
            } else {
                ("Unknown".to_string(), "".to_string())
            };
            let likes_count = likes_data.iter().filter(|l| l.comment_id == c.id).count() as i32;
            let is_liked = viewer_id.as_ref()
                .map(|uid| likes_data.iter().any(|l| l.comment_id == c.id && &l.user_id == uid))
                .unwrap_or(false);
            crate::database::models::CommentResponse {
                id: c.id,
                post_id: c.post_id,
                user_id: c.user_id.clone(),
                user_name: name,
                user_avatar: avatar,
                content: c.content.clone(),
                likes_count,
                is_liked,
                created_at: c.created_at,
            }
        }).collect()
    };

    Json(post_comments).into_response()
}

pub async fn like_comment(
    Path(comment_id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let mut likes = db.comment_likes.lock().unwrap();
    if likes.iter().any(|l| l.comment_id == comment_id && l.user_id == authenticated_id) {
        return (StatusCode::BAD_REQUEST, "Already liked").into_response();
    }
    likes.push(crate::database::models::CommentLike {
        comment_id,
        user_id: authenticated_id,
        created_at: Utc::now(),
    });
    drop(likes);
    db.save_all();
    StatusCode::OK.into_response()
}

pub async fn unlike_comment(
    Path(comment_id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let authenticated_id = match get_user_id_from_token(&headers) {
        Some(uid) => uid,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    {
        let mut likes = db.comment_likes.lock().unwrap();
        likes.retain(|l| !(l.comment_id == comment_id && l.user_id == authenticated_id));
    }
    db.save_all();
    StatusCode::OK.into_response()
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

    let reply = crate::database::models::PostCommentReply {
        id: Uuid::new_v4(),
        comment_id: req.comment_id,
        post_id: id,
        user_id: authenticated_id,
        content: req.content,
        created_at: Utc::now(),
    };

    {
        let mut replies = db.comment_replies.lock().unwrap();
        replies.push(reply.clone());
    }

    db.save_all();
    (StatusCode::CREATED, Json(reply)).into_response()
}

pub async fn get_replies(
    Path(comment_id): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let replies_data: Vec<crate::database::models::PostCommentReply> = {
        let replies = db.comment_replies.lock().unwrap();
        replies.iter()
            .filter(|r| r.comment_id == comment_id)
            .cloned()
            .collect()
    };
    Json(replies_data).into_response()
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

    // Check if post exists
    {
        let posts = db.posts.lock().unwrap();
        if !posts.contains_key(&id) {
            return (StatusCode::NOT_FOUND, "Post not found").into_response();
        }
    }

    let comment_id = Uuid::new_v4();
    let comment = crate::database::models::PostComment {
        id: comment_id,
        post_id: id,
        user_id: authenticated_id,
        content: req.content,
        created_at: Utc::now(),
    };

    {
        let mut comments = db.post_comments.lock().unwrap();
        comments.push(comment.clone());
    }

    // Update post comment count
    {
        let mut posts = db.posts.lock().unwrap();
        if let Some(post) = posts.get_mut(&id) {
            post.comments_count += 1;
        }
    }

    db.save_all();
    (StatusCode::CREATED, Json(comment)).into_response()
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

    let mut comments = db.post_comments.lock().unwrap();
    let initial_len = comments.len();
    
    let mut authed = false;
    if let Some(comment) = comments.iter().find(|c| c.id == comment_id) {
        if comment.user_id == authenticated_id {
            authed = true;
        }
    }

    if !authed {
        return (StatusCode::FORBIDDEN, "Not your comment").into_response();
    }

    comments.retain(|c| c.id != comment_id);
    
    if comments.len() < initial_len {
        drop(comments);
        let mut posts = db.posts.lock().unwrap();
        if let Some(post) = posts.get_mut(&post_id) {
            post.comments_count = (post.comments_count - 1).max(0);
        }
        db.save_all();
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, "Comment not found").into_response()
    }
}
