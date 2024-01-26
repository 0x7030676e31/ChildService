#![feature(async_closure)]

use crate::model::state::PingLoop;

use std::sync::Arc;
use std::env;

use actix_web::web::Data;
use tokio::sync::RwLock;

mod routes;
mod model;

pub type ArcLock<T> = Arc<RwLock<T>>;
pub type AppState = ArcLock<State>;

pub use model::state::State;

const INNER_PORT: u16 = 2137;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "info");
  }
  
  pretty_env_logger::init();

  log::info!("Starting server on port {}...", INNER_PORT);

  let state = State::new();
  let state = Arc::new(RwLock::new(state));
  state.start_ping_loop();
  
  let server = actix_web::HttpServer::new(move || {
    actix_web::App::new()
      .wrap(actix_cors::Cors::permissive())
      .app_data(Data::new(state.clone()))
      .service(routes::routes())
  });

  server.bind(("0.0.0.0", INNER_PORT))?.run().await
}