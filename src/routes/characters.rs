use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Character {
    id: u32,
    name: String,
}

async fn characters() -> Json<Vec<Character>> {
    Json(vec![
        Character {
            id: 1,
            name: "Finn".into(),
        },
        Character {
            id: 2,
            name: "Jake".into(),
        },
    ])
}

pub fn routes() -> Router {
    Router::new().route("/", get(characters))
}
