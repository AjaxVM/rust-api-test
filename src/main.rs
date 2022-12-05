use axum::Router;
use std::net::SocketAddr;

pub mod utils;

mod routes;

#[tokio::main]
async fn main() {
  let app = Router::new()
    .merge(routes::get_router());

  let addr = SocketAddr::from(([127,0,0,1], 3000));
  println!("Serving on localhost:3000");
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}