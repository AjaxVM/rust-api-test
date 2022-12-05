use axum::{
  routing::{
    get,
    post
  },
  Router,
  Json,
  response::IntoResponse
};
use serde::{
  Deserialize,
  Serialize
};
use crate::utils::route;

#[derive(Serialize)]
struct RootResponse {
  message: String
}

pub fn get_root() -> Router {
  async fn handler() -> impl IntoResponse {
    let output = RootResponse {
      message: "Hello World!".to_owned()
    };

    Json(output)
  }

  route("/", get(handler))
}

#[derive(Deserialize)]
struct PostShape {
  message: String,
  song: Option<String>
}

pub fn post_root() -> Router {
  async fn handler(Json(payload): Json<PostShape>) -> impl IntoResponse {
    println!("Song is: {}", payload.song.unwrap_or("(none)".to_owned()));
    let output = RootResponse {
      message: payload.message
    };

    Json(output)
  }

  route("/", post(handler))
}

pub fn get_router() -> Router {
  Router::new()
    .merge(get_root())
    .merge(post_root())
}