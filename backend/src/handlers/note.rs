use crate::{
    models::{
        auth::AuthUser,
        note::{CreateNotePayload, NoteResponse, UpdateNotePayload, Note},
    },
    repo::note_repo::{NoteRepository},
    routes::AppState,
};
use axum::{
    Extension, Json,
    extract::{Path, State},
};

use crate::response::ApiResponse;

pub async fn create(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(notebook_id): Path<i32>,
    Json(payload): Json<CreateNotePayload>,
) -> ApiResponse<NoteResponse> {
    match NoteRepository::create(&state.pool, &payload, auth_user.id, notebook_id).await {
        Ok(response) => ApiResponse::SucessWithData("Note created".to_string(), response),
        Err(e) => ApiResponse::Failed(format!(
            "Failed to create, [DatabaseError] = {e}"
        )),
    }
}

pub async fn update(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path((notebook_id, note_id)): Path<(i32, i32)>,
    Json(payload): Json<UpdateNotePayload>,
) -> ApiResponse<NoteResponse> {
    match NoteRepository::update(&state.pool, &payload, auth_user.id, notebook_id, note_id).await
    {
        Ok(response) => ApiResponse::SucessWithData("Note updated".to_string(), response),
        Err(e) => ApiResponse::Failed(format!("Failed to change note, [DatabaseError] = {e}")),
    }
}

pub async fn list(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(notebook_id): Path<i32>,
) -> ApiResponse<Vec<Note>> {
    match NoteRepository::get_notes(&state.pool, auth_user.id, notebook_id).await {
        Ok(notes) => ApiResponse::SucessWithData("Notes Fetched".to_string(), notes),
        Err(e) => ApiResponse::Failed(format!("Failed to fetch notes, [DatabaseError] = {e}")),
    }
}

pub async fn get_note(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path((notebook_id, note_id)): Path<(i32, i32)>,
) -> ApiResponse<Note> {
    match NoteRepository::get_note(&state.pool, auth_user.id, notebook_id, note_id).await {
        Ok(note) => ApiResponse::SucessWithData("Note Fetched".to_string(), note),
        Err(e) => ApiResponse::Failed(format!("Failed to fetch note, [DatabaseError] = {e}")),
    }
}

pub async fn delete(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path((notebook_id, note_id)): Path<(i32, i32)>,
) -> ApiResponse<NoteResponse> {
    match NoteRepository::delete(&state.pool, auth_user.id, notebook_id, note_id).await {
        Ok(response) => ApiResponse::SucessWithData("Note deleted!".to_string(), response),
        Err(e) => ApiResponse::Failed(format!("Failed to delete a note, [DatabaseError] = {e}")),
    }
}
