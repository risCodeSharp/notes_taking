use sqlx::PgPool;
use crate::models::{CreateNotebookPayload, NotebookResponse};

pub struct NotebookRepository;

impl NotebookRepository {
    pub async fn create(pool: &PgPool, payload: &CreateNotebookPayload, user_id: i32) ->  Result<NotebookResponse, sqlx::Error>{
        sqlx::query_as::<_, NotebookResponse>(
            "INSERT INTO notebooks(owner_id, name) VALUES ($1, $2) RETURNING id, name"
        )
        .bind(user_id)
        .bind(&payload.name)
        .fetch_one(pool)
        .await
    }
}