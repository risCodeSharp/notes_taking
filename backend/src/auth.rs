use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind};
use crate::{models::auth::{AuthUser, Claims}, response::ApiResponse, routes::AppState};

// i have set token time to 2 hours
pub fn make_token(id: &i32, email: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let exp: usize = (Utc::now() + Duration::hours(2)).timestamp() as usize;
    let claims = Claims {
        sub: id.clone(),
        email: email.to_string(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    )
    
}

pub fn validate_token(token: &str, secret: &str) -> Result<AuthUser, jsonwebtoken::errors::Error>{
 let decoding_key = DecodingKey::from_secret(secret.as_bytes());

 decode::<Claims>(token, &decoding_key, &Validation::default())
    .map(|data| AuthUser {
        id: data.claims.sub,
        email: data.claims.email,
    })
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiResponse<()>> {
    let token = request 
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(ApiResponse::Unauthorized(None))?;

    let user = validate_token(token, &state.jwt_secret) 
        .map_err(|e| match e.kind() {
            ErrorKind::ExpiredSignature => {
                ApiResponse::Unauthorized(Some("Token expired".to_string()))
            },
            ErrorKind::InvalidToken => {
                ApiResponse::Unauthorized(Some("Invalid token".to_string()))
            },
            _ => ApiResponse::Unauthorized(Some("Authentication failed".to_string()))
        })?;
        request.extensions_mut().insert(user);

        Ok(next.run(request).await)
}