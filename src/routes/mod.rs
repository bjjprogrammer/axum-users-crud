pub mod users;

use axum::{Router, routing::get};

use crate::handlers::health::check_alive;
use crate::state::AppState;

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/checkAlive", get(check_alive))
        .nest("/users", users::router())
        .with_state(state)
}