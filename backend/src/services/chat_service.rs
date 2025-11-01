
use crate::utils::state::AppState;
use std::sync::Arc;
use uuid::Uuid;

pub struct ChatService;

impl ChatService {
    pub fn generate_user_id() -> String {
        Uuid::new_v4().to_string()
    }

    pub async fn add_user(state: &Arc<AppState>, user_id: &str) {
        let mut users = state.users.lock().await;
        users.push(user_id.to_string());
    }

    pub async fn remove_user(state: &Arc<AppState>, user_id: &str) {
        let mut users = state.users.lock().await;
        users.retain(|u| u != user_id);
    }
}
