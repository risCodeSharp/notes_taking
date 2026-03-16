use crate::{auth, handlers};
use axum::routing::{delete, get, post, put};
use axum::{Router, middleware};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;
use tower_http::cors::{Any, CorsLayer};
use axum::http::{header, Method};
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub jwt_secret: String,
}

fn public_routes() -> Router<AppState> {
    return Router::new()
        .route("/users/register", post(handlers::auth::create_user))
        .route("/users/login", post(handlers::auth::login_user));
}

fn protected_routes(state: &AppState) -> Router<AppState> {
    return Router::new()
        // auth routes
        .route("/users/me", get(handlers::auth::me))

        // user routes 
        .route("/users/me", put(handlers::user::update))
        .route("/users/me/password", put(handlers::user::update_password))
        .route("/users/me", delete(handlers::user::delete))
        
        // .route
        // notebooks routes
        .route("/notebooks", post(handlers::notebook::create))
        .route("/notebooks", get(handlers::notebook::list))
        .route(
            "/notebooks/{notebook_id}",
            get(handlers::notebook::get_notebook),
        )
        .route("/notebooks/{notebook_id}", put(handlers::notebook::update))
        .route(
            "/notebooks/{notebook_id}",
            delete(handlers::notebook::delete),
        )
        // notes routes
        .route(
            "/notebooks/{notebook_id}/notes",
            post(handlers::note::create),
        )
        .route(
            "/notebooks/{notebook_id}/notes/{note_id}",
            put(handlers::note::update),
        )
        .route(
            "/notebooks/{notebook_id}/notes/{note_id}",
            delete(handlers::note::delete),
        )
        .route("/notebooks/{notebook_id}/notes", get(handlers::note::list))
        .route(
            "/notebooks/{notebook_id}/notes/{note_id}",
            get(handlers::note::get_note),
        )
        .with_state(state.clone())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::auth_middleware,
        ));
}

pub async fn routes_init() -> Router {
    /*
     let allowed_origins = ["http://localhost:4000".parse().unwrap()];

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins) // Allow only localhost:4000
        .allow_methods(vec![Method::GET, Method::POST]);

     */

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ]);
        
    let state = init_app_state().await;
    let public = public_routes();
    let protected = protected_routes(&state);

    Router::<AppState>::new()
        .nest("/api", public)
        .nest("/api", protected)
        .route("/health", get(|| async { "Ok" }))
        .with_state(state)
        .layer(cors)
}

async fn init_app_state() -> AppState {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "super-secret_key".to_string());

    println!("Connecting to database...");
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

    AppState { pool, jwt_secret }
}
