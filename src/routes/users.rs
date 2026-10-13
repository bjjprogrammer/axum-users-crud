use axum::{Router, routing::get};

use crate::handlers::users::{create_user, delete_user, get_user, list_users, update_user};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user))
}