use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::error::AppError;
use crate::models::user::{CreateUser, UpdateUser, User};
use crate::services::users as service;
use crate::state::AppState;

pub async fn list_users(State(state): State<AppState>) -> Json<Vec<User>> {
    Json(service::list(&state.users))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<User>, AppError> {
    service::get(&state.users, id).map(Json)
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let user = service::create(&state.users, input)?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<UpdateUser>,
) -> Result<Json<User>, AppError> {
    service::update(&state.users, id, input).map(Json)
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, AppError> {
    service::delete(&state.users, id)?;
    Ok(StatusCode::NO_CONTENT)
}