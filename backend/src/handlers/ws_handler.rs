use crate::{services::chat_service::ChatService, utils::state::AppState};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use std::sync::Arc; // Add this import

pub async fn ws_handler(ws: WebSocketUpgrade, state: Arc<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let user_id = ChatService::generate_user_id();
    ChatService::add_user(&state, &user_id).await;

    let join_msg = format!("🟢 {} joined the chat", &user_id[..8]);
    let _ = state.tx.send(join_msg);

    let mut rx = state.tx.subscribe();
    let tx_clone = state.tx.clone();
    let user_id_clone = user_id.clone();
    let state_clone = state.clone();

    let (mut sender, mut receiver) = socket.split();

    // Spawn a task to handle outgoing messages (broadcast)
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Spawn a task to handle incoming messages (from client)
    let recv_task = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(Message::Text(text)) => {
                    let formatted = format!("💬 {}: {}", &user_id[..8], text);
                    let _ = state.tx.send(formatted);
                }
                Ok(_) => {
                    // ignore non-text
                }
                Err(_) => {
                    break;
                }
            }
        }
    });

    // 👇 Wait until one of the tasks completes (disconnect or error)
    tokio::select! {
        _ = send_task => (),
        _ = recv_task => (),
    }

    // Cleanup after disconnect
    ChatService::remove_user(&state_clone, &user_id_clone).await;
    let leave_msg = format!("🔴 {} left the chat", &user_id_clone[..8]);
    let _ = tx_clone.send(leave_msg);
}


