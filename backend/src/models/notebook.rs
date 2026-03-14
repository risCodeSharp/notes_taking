use serde::{Serialize, Deserialize};
use sqlx::{prelude::FromRow};


#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct NotebookResponse {
    pub id: i32,
    pub name: String,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct NotebookPayload {
    pub name: String,
}
