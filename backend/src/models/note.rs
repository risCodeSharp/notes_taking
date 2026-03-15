use serde::{Serialize, Deserialize};
use sqlx::{prelude::FromRow};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Clone, FromRow)] 
pub struct Note {
    pub id: i32,
    pub owner_id: i32,
    pub title: String,
    pub content: String,
    pub visibility: String,
    pub last_editor_id: i32,
    pub updated_at: NaiveDateTime
}


#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct NoteResponse {
    pub id: i32,
    pub title: String,
    pub notebook_id: i32,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct CreateNotePayload {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateNotePayload {
    pub title: Option<String>,
    pub content: Option<String>,
    pub visibility: Option<String>,
    pub notebook_id: Option<i32>,
}

