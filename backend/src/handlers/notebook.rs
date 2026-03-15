use crate::{
    models::{
        auth::AuthUser,
        notebook::{NotebookPayload, NotebookResponse},
    },
    repo::notebook_repo::NotebookRepository,
    routes::AppState,
};
use axum::{Extension, Json, extract::{Path, State}};

use crate::response::ApiResponse;

pub async fn create(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(payload): Json<NotebookPayload>,
) -> ApiResponse<NotebookResponse> {
    match NotebookRepository::create(&state.pool, &payload, auth_user.id).await {
        Ok(response) => ApiResponse::SuccessWithData("Notebook created".to_string(), response),
        Err(e) => ApiResponse::Failed(format!("Failed to create Notebook, [DatabaseError] = {e}")),
    }
}

pub async fn update ( 
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(notebook_id): Path<i32>,
    Json(payload): Json<NotebookPayload>,
) -> ApiResponse<NotebookResponse> {
    match NotebookRepository::update(&state.pool, &payload, auth_user.id, notebook_id).await {
        Ok(response) => ApiResponse::SuccessWithData("Notebook updated".to_string(), response),
        Err(e) => ApiResponse::Failed(format!("Failed to update Notebook, [DatabaseError] = {e}")),
    }
}

pub async fn delete (
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(notebook_id): Path<i32>,
) -> ApiResponse<NotebookResponse> {
    match NotebookRepository::delete(&state.pool, auth_user.id, notebook_id).await {
        Ok(response) => ApiResponse::SuccessWithData("Notebook deleted".to_string(), response),
        Err(e) => ApiResponse::Failed(format!("Failed to delete notebook, [DatabaseError] = {e}")),
    }
}

pub async fn list(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> ApiResponse<Vec<NotebookResponse>> {
    match NotebookRepository::get_notebooks(&state.pool, auth_user.id).await {
        Ok(response) => ApiResponse::SuccessWithData("All Notebook are Fetched!".to_string(), response),
        Err(e) => ApiResponse::Failed(format!("Failed to fetch all notebook, [DatabaseError] = {e}")),
    }
}

pub async fn get_notebook(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(notebook_id): Path<i32>
) -> ApiResponse<NotebookResponse> {
    match NotebookRepository::get_notebook(&state.pool, auth_user.id, notebook_id).await {
        Ok(response) => ApiResponse::SuccessWithData("Notebook is Fetched!".to_string(), response),
        Err(e) => ApiResponse::Failed(format!("Failed to fetch a notebook, [DatabaseError] = {e}")),
    }
}
