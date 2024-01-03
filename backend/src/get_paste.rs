use axum::{
    response::IntoResponse,
    extract::Path,
    Json
};
use serde::Deserialize;
use crate::db;
use mysql::{prelude::Queryable, Row};

#[derive(Deserialize, Debug)]
pub struct PathParam {
    id: i32
}

pub async fn handler(Path(params): Path<PathParam>) -> impl IntoResponse {
    let mut conn = db::get_conn();

    let mut lang = String::new();
    let mut paste = String::new();
    let id = params.id;

    let query = format!("SELECT lang, paste FROM pastes WHERE id = {}", id);
    let rows: Vec<Row> = conn.query(query.as_str()).unwrap();
    for row in rows {
        lang += &row.get::<String, _>(0).unwrap();
        paste += &row.get::<String, _>(1).unwrap();
    }

    Json(serde_json::json!({
       "lang": lang,
       "paste": paste
    }))
}