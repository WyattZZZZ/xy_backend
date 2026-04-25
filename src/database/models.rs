use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgRow;
use sqlx::Row;

// --- Core Models (Tables) ---

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    User,
    Admin,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Coordinates {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub avatar: String,
    pub gender: String,
    pub bio: String,
    pub location: String,
    pub posts: i32,
    pub following: i32,
    pub fans: i32,
    pub rating: f32,
    pub reviews_count: i32,
    pub coordinates: Option<Coordinates>,
    pub is_verified: bool,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
    pub id: Uuid,
    pub sender: Option<String>,   // User ID or None if deleted
    pub receiver: Option<String>, // User ID or None if deleted
    pub muteby: i8,              // 0: none, 1: sender, 2: receiver, 3: both
    pub last_message: String,
    pub last_message_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: Uuid,
    pub creator_id: String,
    pub avatar: String,
    pub max_member: i32,
    pub member_num: i32,
    pub name: String, // Requested for display
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    pub group_id: Uuid,
    pub user_id: String,
    pub join_time: DateTime<Utc>,
    pub role: GroupRole,
    pub mute_until: Option<DateTime<Utc>>,
    pub is_muted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GroupRole {
    Admin,
    Normal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: String,
    pub reply_id: Option<Uuid>,
    pub content: String,
    pub msg_type: MessageType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageType {
    Text,
    Image,
    Voice,
    Video,
    ProductShare,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    pub follower_id: String,
    pub following_id: String,
    pub created_at: DateTime<Utc>,
}

// --- User DTOs ---

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub avatar: String,
    pub gender: String,
    pub bio: String,
    pub location: String,
    pub posts: i32,
    pub following: i32,
    pub fans: i32,
    pub rating: f32,
    pub reviews_count: i32,
    pub coordinates: Option<Coordinates>,
    pub is_verified: bool,
    pub is_online: bool,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            avatar: user.avatar,
            gender: user.gender,
            bio: user.bio,
            location: user.location,
            posts: user.posts,
            following: user.following,
            fans: user.fans,
            rating: user.rating,
            reviews_count: user.reviews_count,
            coordinates: user.coordinates,
            is_verified: user.is_verified,
            is_online: false,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub gender: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub posts: Option<i32>,
    pub following: Option<i32>,
    pub fans: Option<i32>,
    pub rating: Option<f32>,
    pub reviews_count: Option<i32>,
    pub coordinates: Option<Coordinates>,
    pub is_verified: Option<bool>,
    pub role: Option<Role>,
}

#[derive(Deserialize)]
pub struct UserFilter {
    pub role: Option<Role>,
    pub sort: Option<String>,
    pub query: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationFilter {
    pub user_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupFilter {
    pub user_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowFilter {
    pub follower_id: Option<String>,
    pub following_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowUserItem {
    pub id: String,
    pub name: String,
    pub avatar: String,
}

// --- Social DTOs ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversationRequest {
    pub receiver_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGroupRequest {
    pub name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupRequest {
    pub name: String,
    pub participants: Vec<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WsPayload {
    #[serde(rename_all = "camelCase")]
    SendMessage {
        target_id: Uuid,
        #[serde(default)]
        is_group: bool,
        content: String,
        msg_type: MessageType,
        reply_id: Option<Uuid>,
    },
    #[serde(rename_all = "camelCase")]
    RecallMessage {
        message_id: Uuid,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupMessage {
    pub id: Uuid,
    pub group_id: Uuid,
    pub sender_id: String,
    pub reply_id: Option<Uuid>,
    pub content: String,
    pub msg_type: MessageType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMemberAction {
    pub target_id: String,
    pub leave: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub id: Uuid,
    pub seller_id: String,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub location: String,
    pub coordinates: Option<Coordinates>,
    pub category: String,
    pub tags: Vec<String>,
    pub images: Vec<String>,
    pub video: Option<String>,
    pub condition: String,
    pub status: ProductStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub category_id: String,
    pub english: String,
    pub chinese: String,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductStatus {
    Active,
    Sold,
    Delisted,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductRequest {
    pub title: String,
    pub description: String,
    pub price: f64,
    pub location: String,
    pub coordinates: Option<Coordinates>,
    pub category: String,
    pub tags: Vec<String>,
    pub images: Vec<String>,
    pub condition: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductFilter {
    pub category: Option<String>,
    pub seller_id: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub query: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub radius_km: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: Uuid,
    pub author_id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub location: String,
    pub tags: Vec<String>,
    pub media: Vec<String>,
    pub likes_count: i32,
    pub comments_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub category: String,
    pub location: String,
    pub tags: Vec<String>,
    pub media: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostLike {
    pub post_id: Uuid,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostComment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommentResponse {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: String,
    pub user_name: String,
    pub user_avatar: String,
    pub content: String,
    pub likes_count: i32,
    pub is_liked: bool,
    pub replies: Vec<ReplyResponse>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReplyResponse {
    pub id: Uuid,
    pub comment_id: Uuid,
    pub user_id: String,
    pub user_name: String,
    pub user_avatar: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommentLike {
    pub comment_id: Uuid,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostReplyRequest {
    pub comment_id: Uuid,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostCommentReply {
    pub id: Uuid,
    pub comment_id: Uuid,
    pub post_id: Uuid,
    pub user_id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductFavorite {
    pub product_id: Uuid,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostFavorite {
    pub post_id: Uuid,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCommentRequest {
    pub content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostFilter {
    pub category: Option<String>,
    pub author_id: Option<String>,
    pub query: Option<String>,
}

// --- Auth DTOs ---

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String, 
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: AuthUserResponse,
}

#[derive(serde::Serialize)]
pub struct AuthUserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatListItem {
    pub id: Uuid,
    pub name: String,
    pub avatar: String,
    pub last_msg: String,
    pub time: String,
    pub unread: bool,
    pub is_group: bool,
    pub other_user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Report {
    pub id: Uuid,
    pub target_id: String,
    pub target_type: String, // "post" or "product"
    pub reason: String,
    pub details: Option<String>,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReportRequest {
    pub target_id: String,
    pub target_type: String,
    pub reason: String,
    pub details: Option<String>,
}

// --- DB Row helpers ---

pub fn user_from_row(row: &PgRow) -> User {
    let lat: Option<f64> = row.get("lat");
    let lng: Option<f64> = row.get("lng");
    let role_str: String = row.get("role");
    User {
        id: row.get("id"),
        email: row.get("email"),
        password_hash: row.get("password_hash"),
        name: row.get("name"),
        avatar: row.get("avatar"),
        gender: row.get("gender"),
        bio: row.get("bio"),
        location: row.get("location"),
        posts: row.get("posts"),
        following: row.get("following"),
        fans: row.get("fans"),
        rating: row.get("rating"),
        reviews_count: row.get("reviews_count"),
        coordinates: match (lat, lng) {
            (Some(lat), Some(lng)) => Some(Coordinates { lat, lng }),
            _ => None,
        },
        is_verified: row.get("is_verified"),
        role: if role_str == "ADMIN" { Role::Admin } else { Role::User },
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub fn conversation_from_row(row: &PgRow) -> Conversation {
    Conversation {
        id: row.get("id"),
        sender: row.get("sender"),
        receiver: row.get("receiver"),
        muteby: row.get::<i16, _>("muteby") as i8,
        last_message: row.get("last_message"),
        last_message_at: row.get("last_message_at"),
        created_at: row.get("created_at"),
    }
}

pub fn group_from_row(row: &PgRow) -> Group {
    Group {
        id: row.get("id"),
        creator_id: row.get("creator_id"),
        avatar: row.get("avatar"),
        max_member: row.get("max_member"),
        member_num: row.get("member_num"),
        name: row.get("name"),
        created_at: row.get("created_at"),
    }
}

pub fn group_member_from_row(row: &PgRow) -> GroupMember {
    let role_str: String = row.get("role");
    GroupMember {
        group_id: row.get("group_id"),
        user_id: row.get("user_id"),
        join_time: row.get("join_time"),
        role: if role_str == "ADMIN" { GroupRole::Admin } else { GroupRole::Normal },
        mute_until: row.get("mute_until"),
        is_muted: row.get("is_muted"),
    }
}

pub fn message_from_row(row: &PgRow) -> Message {
    let msg_type_str: String = row.get("msg_type");
    Message {
        id: row.get("id"),
        conversation_id: row.get("conversation_id"),
        sender_id: row.get("sender_id"),
        reply_id: row.get("reply_id"),
        content: row.get("content"),
        msg_type: parse_message_type(&msg_type_str),
        created_at: row.get("created_at"),
    }
}

pub fn group_message_from_row(row: &PgRow) -> GroupMessage {
    let msg_type_str: String = row.get("msg_type");
    GroupMessage {
        id: row.get("id"),
        group_id: row.get("group_id"),
        sender_id: row.get("sender_id"),
        reply_id: row.get("reply_id"),
        content: row.get("content"),
        msg_type: parse_message_type(&msg_type_str),
        created_at: row.get("created_at"),
    }
}

pub fn product_from_row(row: &PgRow) -> Product {
    let lat: Option<f64> = row.get("lat");
    let lng: Option<f64> = row.get("lng");
    let status_str: String = row.get("status");
    Product {
        id: row.get("id"),
        seller_id: row.get("seller_id"),
        title: row.get("title"),
        description: row.get("description"),
        price: row.get("price"),
        location: row.get("location"),
        coordinates: match (lat, lng) {
            (Some(lat), Some(lng)) => Some(Coordinates { lat, lng }),
            _ => None,
        },
        category: row.get("category"),
        tags: row.get("tags"),
        images: row.get("images"),
        video: row.get("video"),
        condition: row.get("condition"),
        status: match status_str.as_str() {
            "SOLD" => ProductStatus::Sold,
            "DELISTED" => ProductStatus::Delisted,
            _ => ProductStatus::Active,
        },
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub fn post_from_row(row: &PgRow) -> Post {
    Post {
        id: row.get("id"),
        author_id: row.get("author_id"),
        title: row.get("title"),
        content: row.get("content"),
        category: row.get("category"),
        location: row.get("location"),
        tags: row.get("tags"),
        media: row.get("media"),
        likes_count: row.get("likes_count"),
        comments_count: row.get("comments_count"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub fn parse_message_type(s: &str) -> MessageType {
    match s {
        "IMAGE" => MessageType::Image,
        "VOICE" => MessageType::Voice,
        "VIDEO" => MessageType::Video,
        "PRODUCT_SHARE" => MessageType::ProductShare,
        _ => MessageType::Text,
    }
}

pub fn message_type_str(t: &MessageType) -> &'static str {
    match t {
        MessageType::Text => "TEXT",
        MessageType::Image => "IMAGE",
        MessageType::Voice => "VOICE",
        MessageType::Video => "VIDEO",
        MessageType::ProductShare => "PRODUCT_SHARE",
    }
}
