use crate::{
    auth::make_token,
    models::auth::{AuthResponse, AuthUser, LoginPayload, UserPublic},
    repo::user_repo::UserRepository,
    routes::AppState,
};
use axum::{
    Extension, Json,
    extract::State,
};
use bcrypt::{DEFAULT_COST, hash};

use crate::models::auth::{CreateUserPayload, UserResponse};
use crate::response::ApiResponse;

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> ApiResponse<UserResponse> {
    if UserRepository::exists(&state.pool, &payload.email).await {
        return ApiResponse::<UserResponse>::Failed("User already exists!".to_string());
    }

    let password_hash = hash(&payload.password, DEFAULT_COST).unwrap();
    match UserRepository::create(&state.pool, payload, &password_hash).await {
        Ok(user_response) => {
            ApiResponse::<UserResponse>::SucessWithData("User created!".to_string(), user_response)
        }
        Err(e) => {
            ApiResponse::<UserResponse>::Failed(format!("Failed to create a user, error = {e}"))
        }
    }
}

pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> ApiResponse<AuthResponse> {
    if !UserRepository::exists(&state.pool, &payload.email).await {
        return ApiResponse::<AuthResponse>::Failed("No Account found".to_string());
    }

    let (id, email, password_hash) = match UserRepository::get_user(&state.pool, &payload.email)
        .await
    {
        Ok(v) => v,
        Err(_) => {
            return ApiResponse::<AuthResponse>::InternalServerError("Database error".to_string());
        }
    };

    let valid = match bcrypt::verify(&payload.password, &password_hash) {
        Ok(v) => v,
        Err(_) => return ApiResponse::Unauthorized(None),
    };
    if !valid {
        return ApiResponse::<_>::Unauthorized(None);
    }

    match make_token(&id, &email, &state.jwt_secret) {
        Ok(token) => {
            let user = UserPublic { id, email };
            ApiResponse::SucessWithData("Login successful".into(), AuthResponse { token, user })
        }
        Err(_) => ApiResponse::InternalServerError("Failed to create token".into()),
    }
}

pub async fn me(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> ApiResponse<UserPublic> {
    match UserRepository::get_user(&state.pool, &auth_user.email).await {
        Ok(_response) => ApiResponse::SucessWithData(
            "ok".into(),
            UserPublic {
                id: auth_user.id,
                email: auth_user.email,
            },
        ),
        Err(_) => ApiResponse::Failed("User not found".into()),
    }
}

// pub async fn login_user(Json(payload)) {

// }
