use crate::structs::*;
use std::path::PathBuf;

use anyhow::Result;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use cs381_hw7::*;
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

fn do_question_two(s: Vec<u32>, e: Vec<u32>) -> Result<u32, AppError> {
    Ok(vip_scheduler(&s, &e)?)
}

pub async fn question_two(Json(payload): Json<QuestionTwo>) -> impl IntoResponse {
    match do_question_two(payload.s, payload.e) {
        Ok(result) => (StatusCode::OK, Json(QuestionAnswer { answer: result })).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn question_two_test_cases() -> impl IntoResponse {
    load_test_cases("q2_test_cases.json").await
}

fn do_question_four(p: Vec<u32>, t: Vec<u32>, d: Vec<u32>) -> Result<u32, AppError> {
    Ok(homework_max_points(&p, &t, &d)?)
}

pub async fn question_four(Json(payload): Json<QuestionFour>) -> impl IntoResponse {
    match do_question_four(payload.p, payload.t, payload.d) {
        Ok(result) => (StatusCode::OK, Json(QuestionAnswer { answer: result })).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn question_four_test_cases() -> impl IntoResponse {
    load_test_cases("q4_test_cases.json").await
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
