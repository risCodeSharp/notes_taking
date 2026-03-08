use crate::models::{CreateNotePayload, NoteResponse};
use sqlx::PgPool;

pub struct NoteRepository;

impl NoteRepository {
    pub async fn create(
        pool: &PgPool,
        payload: &CreateNotePayload,
        user_id: i32,
        notebook_id: i32,
    ) -> Result<NoteResponse, sqlx::Error> {
        sqlx::query_as::<_, NoteResponse>(
            "INSERT INTO notes(owner_id, title, notebook_id) VALUES ($1, $2, $3) RETURNING id, title, notebook_id"
        )
        .bind(user_id)
        .bind(&payload.name)
        .bind(&notebook_id)
        .fetch_one(pool)
        .await
    }
}
