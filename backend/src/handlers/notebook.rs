use crate::{
    models::{AuthUser, CreateNotebookPayload, NotebookResponse},
    repo::notebook_repo::NotebookRepository,
    routes::AppState,
};
use axum::{Extension, Json, extract::State};

use crate::response::ApiResponse;

pub async fn create(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(payload): Json<CreateNotebookPayload>,
) -> ApiResponse<NotebookResponse> {
    match NotebookRepository::create(&state.pool, &payload, auth_user.id).await {
        Ok(response) => ApiResponse::SucessWithData("Note created!".to_string(), response),
        Err(e) => ApiResponse::Failed(format!("Failed to create a user, error = {e}")),
    }
}
