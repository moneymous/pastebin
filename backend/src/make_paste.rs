use crate::db;
use axum::{
    extract,
    response::IntoResponse,
    Json
};
use serde::Deserialize;
use mysql::{prelude::Queryable, Row};

#[derive(Debug, Deserialize)]
pub struct PasteForm {
    paste: String,
    lang: String
}

pub async fn handler(extract::Json(payload): extract::Json<PasteForm>) -> impl IntoResponse {
    let mut conn = db::get_conn();

    let mut id = String::new();
    let paste = payload.paste;
    let lang = payload.lang;

    let query = "INSERT INTO pastes (paste, lang) VALUES (?, ?)";
    let _ = conn.exec_drop(query, (paste, lang));

    let query = "SELECT LAST_INSERT_ID()";
    let rows: Vec<Row> = conn.query(query).unwrap();
    for row in rows {
        id = row.get::<String, _>(0).unwrap();
    }

    Json(serde_json::json!({
        "id": id
    }))
}
