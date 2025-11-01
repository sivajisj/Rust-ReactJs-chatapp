use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use crate::routes::create_routes;
use crate::utils::state::AppState;

use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

pub fn create_app()-> Router {
    let (tx, _rx) = broadcast::channel::<String>(100);
    let state = Arc::new(AppState::new(tx, Mutex::new(Vec::new())));
    
    let corse = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    create_routes(state).layer(corse)
}