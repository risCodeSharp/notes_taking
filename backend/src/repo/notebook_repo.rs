use sqlx::PgPool;
use crate::{ models::notebook::{NotebookPayload, NotebookResponse}};

pub struct NotebookRepository;

impl NotebookRepository {
    pub async fn create(pool: &PgPool, payload: &NotebookPayload, user_id: i32) ->  Result<NotebookResponse, sqlx::Error>{
        sqlx::query_as::<_, NotebookResponse>(
            "INSERT INTO notebooks(owner_id, name) VALUES ($1, $2) RETURNING id, name"
        )
        .bind(user_id)
        .bind(&payload.name)
        .fetch_one(pool)
        .await
    }

    pub async fn update(pool: &PgPool, payload: &NotebookPayload, user_id: i32, notebook_id: i32) ->  Result<NotebookResponse, sqlx::Error>{
        sqlx::query_as::<_, NotebookResponse>(
            "UPDATE notebooks
            SET 
                name = $1,
                owner_id = $2 
            WHERE 
                id = $3
            RETURNING id, name"
        )
        .bind(&payload.name)
        .bind(user_id)
        .bind(notebook_id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, user_id: i32, notebook_id: i32) -> Result<NotebookResponse, sqlx::Error>{
        sqlx::query_as::<_, NotebookResponse>(
            "DELETE FROM notebooks 
            WHERE id = $1 AND owner_id = $2
            RETURNING id, name"
        )
        .bind(notebook_id)
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn get_notebook(pool: &PgPool, user_id: i32, notebook_id: i32) -> Result<NotebookResponse, sqlx::Error>{
        sqlx::query_as::<_, NotebookResponse>(
            "SELECT id, name FROM notebooks WHERE owner_id = $1 AND id = $2"
        )
        .bind(user_id)
        .bind(notebook_id)
        .fetch_one(pool)
        .await
    }
    
    pub async fn get_notebooks(pool: &PgPool, user_id: i32) -> Result<Vec<NotebookResponse>, sqlx::Error>{
        sqlx::query_as::<_, NotebookResponse>(
            "SELECT id, name FROM notebooks WHERE owner_id = $1"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }
}