use crate::{
    auth::{self, make_token},
    models::auth::{
        AuthResponse, AuthUser, LoginPayload, UpdatePasswordPayload, UpdateUserPayload, UserPublic,
    },
    repo::user_repo::{UserRepository, VerifyUserPasswordError},
    routes::AppState,
};
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use bcrypt::{DEFAULT_COST, hash};

use crate::models::auth::UserResponse;
use crate::response::ApiResponse;

pub async fn update(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(payload): Json<UpdateUserPayload>,
) -> ApiResponse<UserResponse> {
    match UserRepository::update_user(&state.pool, payload, auth_user.id).await {
        Ok(user_response) => {
            ApiResponse::<UserResponse>::SuccessWithData("User updated".to_string(), user_response)
        }
        Err(e) => {
            ApiResponse::<UserResponse>::Failed(format!("Failed to create a user, error = {e}"))
        }
    }
}

pub async fn update_password(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(payload): Json<UpdatePasswordPayload>,
) -> ApiResponse<UserResponse> {
    if let Err(error) =
        UserRepository::verify_user_password(&state.pool, &payload.old_password, auth_user.id.clone()).await
    {
        return match error {
            VerifyUserPasswordError::AccountNotFound => {
                ApiResponse::Failed("No account found".to_string())
            }
            VerifyUserPasswordError::PasswordNotMatched => {
                ApiResponse::Failed("Wrong password".to_string())
            }
            VerifyUserPasswordError::DatabaseError => {
                ApiResponse::Failed("Internal server error".to_string())
            }
        };
    }

    // hash new password
    let password_hash = match bcrypt::hash(&payload.new_password, bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return ApiResponse::Failed("Failed to hash password".to_string()),
    };


    match UserRepository::update_password(&state.pool, &password_hash, auth_user.id).await {
        Ok(user_response) => ApiResponse::SuccessWithData(
            "User changed password".to_string(),
            user_response,
        ),
        Err(e) => {
            ApiResponse::Failed(format!("Failed to update password, error = {e}"))
        }
    }
}


pub async fn delete(
State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> ApiResponse<UserResponse> {
    match UserRepository::delete(&state.pool, auth_user.id).await {
        Ok(user_response) => ApiResponse::SuccessWithData(
            "Deleted User".to_string(),
            user_response,
        ),
        Err(e) => {
            ApiResponse::Failed(format!("Failed to delete user, error = {e}"))
        }
    }
}