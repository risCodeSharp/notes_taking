use axum::{
    body::Body,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub enum ApiResponse<T: Serialize> {
    Success(String),
    SuccessWithData(String, T),
    Failed(String),
    Unauthorized(Option<String>),
    InternalServerError(String),
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        match self {
            Self::Success(msg) => {
                let json = json!({
                    "sucess": true,
                    "data":{
                        "message": msg,
                    },
                });
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json.to_string()))
                    .unwrap_or_default()
            }
            Self::SuccessWithData(msg, new_user) => {
                let json = json!({
                    "success": true,
                    "data": new_user,
                    "message": msg 
                });
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json.to_string()))
                    .unwrap_or_default()
            }
            Self::Failed(msg) => {
                let json = json!({
                    "success": false,
                    "data": {
                        "message": msg
                    }
                });
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json.to_string()))
                    .unwrap_or_default()
            }
            Self::Unauthorized(msg) => {
                let json = json!({
                    "success": false,
                    "data": {
                        "message": match msg { Some(v) => v, None => "Wrong password!".to_string()}
                    }
                });
                Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json.to_string()))
                    .unwrap_or_default()
            }
            Self::InternalServerError(msg) => {
                let json = json!({
                    "success": false,
                    "data": {
                        "message": msg
                    }
                });
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json.to_string()))
                    .unwrap_or_default()
            }
        }
    }
}
