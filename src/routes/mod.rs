use axum::Router;
mod root;

pub fn get_router() -> Router {
  Router::new()
    .merge(root::get_router())
}