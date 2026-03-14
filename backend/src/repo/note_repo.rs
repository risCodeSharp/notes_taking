use crate::models::note::{CreateNotePayload, Note, NoteResponse, UpdateNotePayload};
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
            r#"
            INSERT INTO notes (
                owner_id,
                title,
                content,
                notebook_id,
                last_editor_id
            ) VALUES ($1, $2, $3, $4, $5) 
            RETURNING id, title, notebook_id"#,
        )
        .bind(&user_id)
        .bind(&payload.title)
        .bind(&payload.content)
        .bind(&notebook_id)
        .bind(&user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn update(
        pool: &PgPool,
        payload: &UpdateNotePayload,
        user_id: i32,
        notebook_id: i32,
        note_id: i32,
    ) -> Result<NoteResponse, sqlx::Error> {
        sqlx::query_as::<_, NoteResponse>(
            r#"
            UPDATE notes
            SET 
                title = COALESCE($1, title),
                content = COALESCE($2, content),
                visibility = COALESCE($3, visibility),
                last_editor_id = $4,
                updated_at = NOW(),
                notebook_id = COALESCE($5, notebook_id)
            WHERE 
                id = $6 AND
                notebook_id = $7 AND
                owner_id = $8
            RETURNING id, title, notebook_id
            "#,
        )
        .bind(&payload.title)
        .bind(&payload.content)
        .bind(&payload.visibilty)
        .bind(&user_id)
        .bind(&payload.notebook_id)
        .bind(&note_id)
        .bind(&notebook_id)
        .bind(&user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(
        pool: &PgPool,
        user_id: i32,
        notebook_id: i32,
        note_id: i32,
    ) -> Result<NoteResponse, sqlx::Error> {
        sqlx::query_as::<_, NoteResponse>(
            "DELETE FROM notes 
            WHERE 
                id = $1 AND
                notebook_id = $2 AND
                owner_id = $3
            RETURNING 
                id, title, notebook_id",
        )
        .bind(note_id)
        .bind(notebook_id)
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn get_note(
        pool: &PgPool,
        user_id: i32,
        notebook_id: i32,
        note_id: i32,
    ) -> Result<Note, sqlx::Error> {
        sqlx::query_as::<_, Note>(
            r#"SELECT id, owner_id, title, content, visibility, last_editor_id, updated_at 
            FROM notes
             WHERE 
                id = $1 AND
                notebook_id = $2 AND
                owner_id = $3
            "#,
        )
        .bind(note_id)
        .bind(notebook_id)
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn get_notes(
        pool: &PgPool,
        user_id: i32,
        notebook_id: i32,
    ) -> Result<Vec<Note>, sqlx::Error> {
        sqlx::query_as::<_, Note>(
            r#"SELECT id, owner_id, title, content, visibility, last_editor_id, updated_at
            FROM notes 
            WHERE 
                notebook_id = $1 AND 
                owner_id = $2 "#,
        )
        .bind(notebook_id)
        .bind(user_id)
        .fetch_all(pool)
        .await
    }
}
