use crate::error::AppError;
use crate::models::user::{CreateUser, UpdateUser, User};
use crate::repositories::users::UserRepository;

pub fn list(repo: &UserRepository) -> Vec<User> {
    repo.find_all()
}

pub fn get(repo: &UserRepository, id: u64) -> Result<User, AppError> {
    repo.find_by_id(id).ok_or(AppError::NotFound)
}

pub fn create(repo: &UserRepository, input: CreateUser) -> Result<User, AppError> {
    validate(&input.name, &input.email)?;
    if repo.find_by_email(&input.email).is_some() {
        return Err(AppError::Conflict("El email ya está registrado".into()));
    }
    Ok(repo.insert(input.name, input.email))
}

pub fn update(repo: &UserRepository, id: u64, input: UpdateUser) -> Result<User, AppError> {
    let mut user = repo.find_by_id(id).ok_or(AppError::NotFound)?;

    if let Some(email) = input.email {
        if repo.find_by_email(&email).is_some_and(|other| other.id != id) {
            return Err(AppError::Conflict("El email ya está registrado".into()));
        }
        user.email = email;
    }
    if let Some(name) = input.name {
        user.name = name;
    }

    validate(&user.name, &user.email)?;
    repo.update(user).ok_or(AppError::NotFound)
}

pub fn delete(repo: &UserRepository, id: u64) -> Result<(), AppError> {
    if repo.delete(id) { Ok(()) } else { Err(AppError::NotFound) }
}

fn validate(name: &str, email: &str) -> Result<(), AppError> {
    if name.trim().is_empty() {
        return Err(AppError::BadRequest("El nombre es obligatorio".into()));
    }
    if !email.contains('@') {
        return Err(AppError::BadRequest("Email inválido".into()));
    }
    Ok(())
}