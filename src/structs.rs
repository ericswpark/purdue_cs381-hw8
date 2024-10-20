use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct QuestionOne {
    pub(crate) t: Vec<u32>,
    pub(crate) d: Vec<u32>,
}

#[derive(Serialize)]
pub struct QuestionAnswer {
    pub(crate) answer: u32,
    pub(crate) answer_ordering: Vec<usize>
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_message = format!("{}", self.0);
        let body = json!({ "error": error_message });
        (StatusCode::BAD_REQUEST, body.to_string()).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
