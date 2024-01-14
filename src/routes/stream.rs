use super::extractors::UserGuard;
use crate::AppState;

use actix_web::{get, web, Responder};

#[get("/stream")]
pub async fn stream(state: web::Data<AppState>, user: UserGuard) -> impl Responder {
  let mut state = state.write().await;
  let user = user.0.read().await;
  
  state.new_stream(user.uuid.clone())
}