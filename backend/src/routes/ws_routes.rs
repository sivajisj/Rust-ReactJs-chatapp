use axum::{routing::get, Router};
use std::sync::Arc;
use crate::{handlers::ws_handler::ws_handler, utils::state::AppState};

pub fn ws_routes(state: Arc<AppState>) -> Router {
    Router::new().route("/ws", get(move |ws| ws_handler(ws, state.clone())))
}
