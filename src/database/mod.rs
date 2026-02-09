pub mod models;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fs;
use serde_json;
use crate::database::models::{User, Conversation, Message, Follow, Group, GroupMember, GroupMessage};
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
    }

    pub fn save_all(&self) {
        let _ = fs::create_dir_all("database");
        
        {
            let users = self.users.lock().unwrap();
            let _ = fs::write("database/users.json", serde_json::to_string_pretty(&*users).unwrap());
        }
        
        {
            let convs = self.conversations.lock().unwrap();
            let _ = fs::write("database/conversations.json", serde_json::to_string_pretty(&*convs).unwrap());
        }

        {
            let groups = self.groups.lock().unwrap();
            let _ = fs::write("database/groups.json", serde_json::to_string_pretty(&*groups).unwrap());
        }

        {
            let members = self.group_members.lock().unwrap();
            let _ = fs::write("database/group_members.json", serde_json::to_string_pretty(&*members).unwrap());
        }
        
        {
            let msgs = self.messages.lock().unwrap();
            let _ = fs::write("database/messages.json", serde_json::to_string_pretty(&*msgs).unwrap());
        }

        {
            let group_msgs = self.group_messages.lock().unwrap();
            let _ = fs::write("database/group_messages.json", serde_json::to_string_pretty(&*group_msgs).unwrap());
        }
        
        {
            let follows = self.follows.lock().unwrap();
            let _ = fs::write("database/follows.json", serde_json::to_string_pretty(&*follows).unwrap());
        }
    }
}
