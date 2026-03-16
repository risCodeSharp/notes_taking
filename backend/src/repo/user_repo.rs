use crate::models::auth::{CreateUserPayload, UpdateUserPayload, UserResponse};
use sqlx::PgPool;

pub enum VerifyUserPasswordError {
    AccountNotFound,
    PasswordNotMatched,
    DatabaseError,
}

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        pool: &PgPool,
        payload: CreateUserPayload,
    ) -> Result<UserResponse, sqlx::Error> {
        let password_hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)
            .map_err(|_| sqlx::Error::RowNotFound)?;
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
        let exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(email)
            .fetch_one(pool)
            .await
            .unwrap();

        exists.0
    }

    pub async fn get_user_by_email(
        pool: &PgPool,
        email: &str,
    ) -> Result<(i32, String, String), sqlx::Error> {
        sqlx::query_as::<_, (i32, String, String)>(
            "SELECT id, email, password_hash FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_one(pool)
        .await
    }

    pub async fn get_user(pool: &PgPool, user_id: i32) -> Result<UserResponse, sqlx::Error> {
        sqlx::query_as::<_, UserResponse>(
            "SELECT id, email, password_hash FROM users WHERE email = $1",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
    }
    pub async fn update_user(
        pool: &PgPool,
        payload: UpdateUserPayload,
        user_id: i32,
    ) -> Result<UserResponse, sqlx::Error> {
        sqlx::query_as::<_, UserResponse>(
            "UPDATE users SET email = COALESCE($1, email), password_hash = COALESCE($2, password_hash) WHERE id = $3 RETURNING id, name, email"
        )
        .bind(payload.email)
        .bind(payload.password)
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn verify_user_password(
        pool: &PgPool,
        password: &str,
        user_id: i32,
    ) -> Result<(), VerifyUserPasswordError> {
        let stored_hash =
            sqlx::query_scalar::<_, String>("SELECT password_hash from users WHERE id = $1")
                .bind(user_id)
                .fetch_one(pool)
                .await
                .map_err(|_| VerifyUserPasswordError::AccountNotFound)?;

        let valid = bcrypt::verify(&password, &stored_hash)
            .map_err(|_| VerifyUserPasswordError::PasswordNotMatched)?;

        if valid {
            Ok(())
        } else {
            Err(VerifyUserPasswordError::PasswordNotMatched)
        }
    }

    pub async fn update_password(
        pool: &PgPool,
        new_password_hash: &str,
        user_id: i32,
    ) -> Result<UserResponse, sqlx::Error> {

        sqlx::query_as::<_, UserResponse>(
            "UPDATE users SET password_hash = $1 WHERE id = $2 RETURNING id, name, email",
        )
        .bind(new_password_hash)
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, user_id: i32) -> Result<UserResponse, sqlx::Error> {
        sqlx::query_as::<_, UserResponse>(
            "DELETE FROM users WHERE id = $1 RETURNING id, name, email",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    // pub async fn check_user_password()
}
