use sqlx::PgPool;
use crate::models::{CreateUserPayload, UserResponse};

pub struct UserRepository;

impl UserRepository {
    pub async fn create(pool: &PgPool, payload: CreateUserPayload, password_hash: &str) ->  Result<UserResponse, sqlx::Error>{
        sqlx::query_as::<_, UserResponse>(
            "INSERT INTO users(name, email, password_hash) VALUES ($1, $2, $3) RETURNING id, name, email"
        )
        .bind(payload.name)
        .bind(payload.email)
        .bind(password_hash)
        .fetch_one(pool)
        .await
    }
    pub async fn exists(pool: &PgPool, email: &str) -> bool {
        let exists: (bool, ) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"
        )
        .bind(email)
        .fetch_one(pool)
        .await
        .unwrap();

        exists.0
    }

    pub async fn get_user(pool: &PgPool, email: &str) -> Result<(i32, String, String), sqlx::Error> {
        let user = sqlx::query_as::<_, (i32, String, String)>(
            "SELECT id, email, password_hash FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}

