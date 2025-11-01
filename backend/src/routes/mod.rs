pub mod ws_routes;

use axum::Router;
use std::sync::Arc;
use crate::utils::state::AppState;
use self::ws_routes::ws_routes;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new().merge(ws_routes(state))
}
