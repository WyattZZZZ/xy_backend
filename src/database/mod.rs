pub mod models;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fs;
use serde_json;
use crate::database::models::{User, Conversation, Message, Follow, Group, GroupMember, GroupMessage, Product, Post, PostLike, PostComment, CommentLike, PostCommentReply, Category};
use uuid::Uuid;
use dashmap::DashMap;
use tokio::sync::mpsc;
use axum::extract::ws::Message as WsMessage;

pub type ActiveConnections = Arc<DashMap<String, mpsc::UnboundedSender<WsMessage>>>;

pub struct Database {
    pub users: Arc<Mutex<HashMap<String, User>>>,
    pub conversations: Arc<Mutex<HashMap<Uuid, Conversation>>>,
    pub groups: Arc<Mutex<HashMap<Uuid, Group>>>,
    pub group_members: Arc<Mutex<Vec<GroupMember>>>,
    pub messages: Arc<Mutex<Vec<Message>>>,
    pub group_messages: Arc<Mutex<Vec<GroupMessage>>>,
    pub follows: Arc<Mutex<Vec<Follow>>>,
    pub products: Arc<Mutex<HashMap<Uuid, Product>>>,
    pub posts: Arc<Mutex<HashMap<Uuid, Post>>>,
    pub post_likes: Arc<Mutex<Vec<PostLike>>>,
    pub post_comments: Arc<Mutex<Vec<PostComment>>>,
    pub comment_likes: Arc<Mutex<Vec<CommentLike>>>,
    pub comment_replies: Arc<Mutex<Vec<PostCommentReply>>>,
    pub categories: Arc<Mutex<HashMap<String, Category>>>,
    pub conns: ActiveConnections,
}

impl Database {
    pub fn new() -> Self {
        let db = Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            conversations: Arc::new(Mutex::new(HashMap::new())),
            groups: Arc::new(Mutex::new(HashMap::new())),
            group_members: Arc::new(Mutex::new(Vec::new())),
            messages: Arc::new(Mutex::new(Vec::new())),
            group_messages: Arc::new(Mutex::new(Vec::new())),
            follows: Arc::new(Mutex::new(Vec::new())),
            products: Arc::new(Mutex::new(HashMap::new())),
            posts: Arc::new(Mutex::new(HashMap::new())),
            post_likes: Arc::new(Mutex::new(Vec::new())),
            post_comments: Arc::new(Mutex::new(Vec::new())),
            comment_likes: Arc::new(Mutex::new(Vec::new())),
            comment_replies: Arc::new(Mutex::new(Vec::new())),
            categories: Arc::new(Mutex::new(HashMap::new())),
            conns: Arc::new(DashMap::new()),
        };
        db.load_all();
        db
    }

    pub fn load_all(&self) {
        if let Ok(data) = fs::read_to_string("database/users.json") {
            if let Ok(map) = serde_json::from_str::<HashMap<String, User>>(&data) {
                *self.users.lock().unwrap() = map;
            }
        }
        if let Ok(data) = fs::read_to_string("database/conversations.json") {
            if let Ok(map) = serde_json::from_str::<HashMap<Uuid, Conversation>>(&data) {
                *self.conversations.lock().unwrap() = map;
            }
        }
        if let Ok(data) = fs::read_to_string("database/groups.json") {
            if let Ok(map) = serde_json::from_str::<HashMap<Uuid, Group>>(&data) {
                *self.groups.lock().unwrap() = map;
            }
        }
        if let Ok(data) = fs::read_to_string("database/group_members.json") {
            if let Ok(vec) = serde_json::from_str::<Vec<GroupMember>>(&data) {
                *self.group_members.lock().unwrap() = vec;
            }
        }
        if let Ok(data) = fs::read_to_string("database/messages.json") {
            if let Ok(vec) = serde_json::from_str::<Vec<Message>>(&data) {
                *self.messages.lock().unwrap() = vec;
            }
        }
        if let Ok(data) = fs::read_to_string("database/group_messages.json") {
            if let Ok(vec) = serde_json::from_str::<Vec<GroupMessage>>(&data) {
                *self.group_messages.lock().unwrap() = vec;
            }
        }
        if let Ok(data) = fs::read_to_string("database/follows.json") {
            if let Ok(vec) = serde_json::from_str::<Vec<Follow>>(&data) {
                *self.follows.lock().unwrap() = vec;
            }
        }
        if let Ok(data) = fs::read_to_string("database/products.json") {
            if let Ok(map) = serde_json::from_str::<HashMap<Uuid, Product>>(&data) {
                *self.products.lock().unwrap() = map;
            }
        }
        if let Ok(data) = fs::read_to_string("database/posts.json") {
            if let Ok(map) = serde_json::from_str::<HashMap<Uuid, Post>>(&data) {
                *self.posts.lock().unwrap() = map;
            }
        }
        if let Ok(data) = fs::read_to_string("database/post_likes.json") {
            if let Ok(vec) = serde_json::from_str::<Vec<PostLike>>(&data) {
                *self.post_likes.lock().unwrap() = vec;
            }
        }
        if let Ok(data) = fs::read_to_string("database/post_comments.json") {
            if let Ok(vec) = serde_json::from_str::<Vec<PostComment>>(&data) {
                *self.post_comments.lock().unwrap() = vec;
            }
        }
        if let Ok(data) = fs::read_to_string("database/comment_likes.json") {
            if let Ok(vec) = serde_json::from_str::<Vec<CommentLike>>(&data) {
                *self.comment_likes.lock().unwrap() = vec;
            }
        }
        if let Ok(data) = fs::read_to_string("database/comment_replies.json") {
            if let Ok(vec) = serde_json::from_str::<Vec<PostCommentReply>>(&data) {
                *self.comment_replies.lock().unwrap() = vec;
            }
        }
        if let Ok(data) = fs::read_to_string("database/categories.json") {
            match serde_json::from_str::<HashMap<String, Category>>(&data) {
                Ok(map) => {
                    *self.categories.lock().unwrap() = map;
                },
                Err(e) => {
                    eprintln!("Failed to parse categories.json: {}", e);
                }
            }
        } else {
            eprintln!("database/categories.json not found");
        }
    }

    pub fn save_all(&self) {
        let _ = fs::create_dir_all("database");

        // Collect all (path, json) pairs while holding each lock BRIEFLY.
        // We NEVER hold more than one lock at a time.
        // We release every lock BEFORE doing any file I/O to avoid deadlocks.
        let mut files: Vec<(&'static str, String)> = Vec::new();

        macro_rules! collect {
            ($field:expr, $path:literal) => {
                if let Ok(json) = serde_json::to_string_pretty(&*$field.lock().unwrap()) {
                    files.push(($path, json));
                }
            };
        }

        collect!(self.users,           "database/users.json");
        collect!(self.conversations,   "database/conversations.json");
        collect!(self.groups,          "database/groups.json");
        collect!(self.group_members,   "database/group_members.json");
        collect!(self.messages,        "database/messages.json");
        collect!(self.group_messages,  "database/group_messages.json");
        collect!(self.follows,         "database/follows.json");
        collect!(self.products,        "database/products.json");
        collect!(self.posts,           "database/posts.json");
        collect!(self.post_likes,      "database/post_likes.json");
        collect!(self.post_comments,   "database/post_comments.json");
        collect!(self.comment_likes,   "database/comment_likes.json");
        collect!(self.comment_replies, "database/comment_replies.json");
        collect!(self.categories,      "database/categories.json");

        // ALL LOCKS ARE NOW RELEASED. Do file I/O without holding any lock.
        for (path, json) in files {
            if let Err(e) = fs::write(path, json) {
                eprintln!("Error writing {}: {}", path, e);
            }
        }
    }
}

            }
        }

        {
            let groups = self.groups.lock().unwrap();
            if let Ok(data) = serde_json::to_string_pretty(&*groups) {
                write_file("database/groups.json", data);
            }
        }

        {
            let members = self.group_members.lock().unwrap();
            if let Ok(data) = serde_json::to_string_pretty(&*members) {
                write_file("database/group_members.json", data);
            }
        }
        
        {
            let msgs = self.messages.lock().unwrap();
            if let Ok(data) = serde_json::to_string_pretty(&*msgs) {
                write_file("database/messages.json", data);
            }
        }

        {
            let group_msgs = self.group_messages.lock().unwrap();
            if let Ok(data) = serde_json::to_string_pretty(&*group_msgs) {
                write_file("database/group_messages.json", data);
            }
        }
        
        {
            let follows = self.follows.lock().unwrap();
            if let Ok(data) = serde_json::to_string_pretty(&*follows) {
                write_file("database/follows.json", data);
            }
        }

        {
            let products = self.products.lock().unwrap();
            if let Ok(data) = serde_json::to_string_pretty(&*products) {
                write_file("database/products.json", data);
            }
        }

        {
            let posts = self.posts.lock().unwrap();
            if let Ok(data) = serde_json::to_string_pretty(&*posts) {
                write_file("database/posts.json", data);
            }
        }

        {
            let likes = self.post_likes.lock().unwrap();
            if let Ok(data) = serde_json::to_string_pretty(&*likes) {
                write_file("database/post_likes.json", data);
            }
        }

        {
            let comments = self.post_comments.lock().unwrap();
            if let Ok(data) = serde_json::to_string_pretty(&*comments) {
                write_file("database/post_comments.json", data);
            }
        }

        {
            let likes = self.comment_likes.lock().unwrap();
            if let Ok(data) = serde_json::to_string_pretty(&*likes) {
                write_file("database/comment_likes.json", data);
            }
        }

        {
            let replies = self.comment_replies.lock().unwrap();
            if let Ok(data) = serde_json::to_string_pretty(&*replies) {
                write_file("database/comment_replies.json", data);
            }
        }

        {
            let categories = self.categories.lock().unwrap();
            if let Ok(data) = serde_json::to_string_pretty(&*categories) {
                write_file("database/categories.json", data);
            }
        }
    }
}
