use crate::{AppState, State, ArcLock};
use crate::model::user;

use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use futures::future;

#[derive(Serialize, Deserialize)]
struct LoginCredentials {
  pub username: String,
  pub password: String,
}

#[actix_web::post("/register")]
async fn register(state: web::Data<AppState>, body: web::Json<LoginCredentials>) -> HttpResponse {
  let mut state = state.write().await;
  
  let users = state.users.values().map(|user| user.read());
  let users = future::join_all(users).await;

  if users.iter().any(|user| user.username == body.username) {
    return HttpResponse::Conflict().finish();
  }

  drop(users);
  let LoginCredentials { username, password } = body.into_inner();
  log::info!("Registering user {}", username);
  
  let user = user::User::new(username, password);
  let token = State::generate_epicos_tokens();
  state.users.insert(token.clone(), ArcLock(user));

  HttpResponse::Ok().body(token)
}

#[actix_web::post("/login")]
async fn login(state: web::Data<AppState>, body: web::Json<LoginCredentials>) -> HttpResponse {
  let state = state.read().await;
  let LoginCredentials { username, password } = body.into_inner();

  let users = state.users.iter().map(async move |(token, user)| (token, user.read().await));
  let user = future::join_all(users).await.into_iter().find(|(_, user)| user.username == username && user.password == password);

  match user {
    Some((token, _)) => HttpResponse::Ok().body(token.clone()),
    None => HttpResponse::Unauthorized().finish(),
  }
}

pub fn routes() -> Scope {
  Scope::new("/users")
    .service(register)
}