use serde::{Serialize, Deserialize};
use sqlx::{prelude::FromRow};

// #[derive(Serialize, Deserialize)]
// pub struct User {
//     pub name: String,
//     pub email: String,
//     pub password_hash: String,
// }

#[derive(Serialize, Deserialize)]
pub struct CreateUserPayload {
    pub name: String,
    pub email: String,
    pub password: String,
}


#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
}


#[derive(Serialize, Deserialize, FromRow)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserPublic,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPublic {
    pub id: i32,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,
    pub email: String,
    pub exp: usize,
}

#[derive(Clone)] 
pub struct AuthUser {
    pub id: i32,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct NotebookResponse {
    pub id: i32,
    pub name: String,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct CreateNotebookPayload {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct NoteResponse {
    pub id: i32,
    pub name: String,
    pub notebook_id: i32,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct CreateNotePayload {
    pub name: String,
}

