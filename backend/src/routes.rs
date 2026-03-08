use axum::{Router, middleware};
use axum::routing::{get, post};
use std::env;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::{auth, handlers};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub jwt_secret: String,
}

fn public_routes() -> Router<AppState> {
    return Router::new()
    .route("/user/register", post(handlers::users::create_user))
    .route("/user/login", post(handlers::users::login_user))
    
}

fn protected_routes(state: &AppState) -> Router<AppState> {
    return Router::new()
    .route("/user/me", get(handlers::users::me))
    .route("/notebook", post(handlers::notebook::create))
    .route("/notebook/{notebook_id}/note", post(handlers::note::create))
    .with_state(state.clone())
    .layer(middleware::from_fn_with_state(state.clone(), auth::auth_middleware))
    
}

pub async fn routes_init() -> Router {

    let state = init_app_state().await;
    let public = public_routes();
    let protected = protected_routes(&state);

    Router::<AppState>::new()
    .nest("/api", public)
    .nest("/api", protected)
    .with_state(state)
}


async fn init_app_state() -> AppState {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {"super-secret_key".to_string()});

    let pool: Pool<Postgres> = PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
    .expect("Failed to connect to database");

    // Runs migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");


    AppState {
        pool,
        jwt_secret    
    }
}

