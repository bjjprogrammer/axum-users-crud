use std::sync::Arc;

use crate::repositories::users::UserRepository;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<UserRepository>,
}