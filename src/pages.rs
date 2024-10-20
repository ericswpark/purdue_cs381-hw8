use crate::structs::*;
use std::path::PathBuf;

use anyhow::Result;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use cs381_hw8::*;
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

fn do_question_one(t: Vec<u32>, d: Vec<u32>) -> Result<(Vec<usize>, Vec<usize>, u32), AppError> {
    Ok(starbucks_scheduler(&t, &d)?)
}

pub async fn question_one(Json(payload): Json<QuestionOne>) -> impl IntoResponse {
    match do_question_one(payload.t, payload.d) {
        Ok(result) => (StatusCode::OK, Json(QuestionAnswer { answer: result.2, answer_ordering: result.0, answer_process_ordering: result.1 })).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn question_one_test_cases() -> impl IntoResponse {
    load_test_cases("q1_test_cases.json").await
}

async fn load_test_cases(file_path: &str) -> impl IntoResponse {
    let file = PathBuf::from(file_path);

    match load_file(file).await {
        Ok(content) => Json(serde_json::from_str::<Value>(&content).unwrap()).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error loading test cases: {}", err),
        )
            .into_response(),
    }
}

async fn load_file(path: PathBuf) -> Result<String, std::io::Error> {
    let mut file = File::open(path).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    Ok(content)
}
