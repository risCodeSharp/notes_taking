use crate::{
    models::{AuthUser, CreateNotePayload, NoteResponse},
    repo::note_repo::NoteRepository,
    routes::AppState,
};
use axum::{Extension, Json, extract::{Path, State}};

use crate::response::ApiResponse;

#[axum::debug_handler]
pub async fn create(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(notebook_id) : Path<i32>,
    Json(payload): Json<CreateNotePayload>,
) -> ApiResponse<NoteResponse> {
    match NoteRepository::create(&state.pool, &payload, auth_user.id, notebook_id).await {
        Ok(response) => ApiResponse::SucessWithData("Note created!".to_string(), response),
        Err(e) => ApiResponse::Failed(format!("Failed to create a note for notebook, error = {e}")),
    }
}
