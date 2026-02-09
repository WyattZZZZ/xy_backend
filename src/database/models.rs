use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

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
