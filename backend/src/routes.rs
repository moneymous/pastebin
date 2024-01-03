use axum::{
    Router,
    routing::{get, post}
};
use crate::get_paste;
use crate::make_paste;

pub fn get_all_routes() -> Router {
    Router::new()
        .route("/get-paste/id/:id", get(get_paste::handler))
        .route("/make-paste", post(make_paste::handler))
}