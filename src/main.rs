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

#[allow(non_snake_case)]
pub fn ArcLock<T>(value: T) -> ArcLock<T> {
  Arc::new(RwLock::new(value))
}

const INNER_PORT: u16 = 2137;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  env::set_var("RUST_LOG", "INFO");
  pretty_env_logger::init();

  log::info!("Starting server on port {}...", INNER_PORT);

  let server = actix_web::HttpServer::new(move || {    
    let state = ArcLock(State::new());
    state.start_ping_loop();

    actix_web::App::new()
      .app_data(Data::new(state))
      .service(routes::routes())
  });

  server.bind(("0.0.0.0", INNER_PORT))?.run().await
}