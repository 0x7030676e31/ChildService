use crate::{AppState, State};
use crate::model::user;

use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LoginCredentials {
  pub username: String,
  pub password: String,
}

#[actix_web::post("/register")]
async fn register(state: web::Data<AppState>, body: web::Json<LoginCredentials>) -> HttpResponse {
  let mut state = state.write().await;
  
  log::info!("Registering user \"{}\"", body.username);

  if state.users.values().any(|user| user.username == body.username) {
    log::info!("User \"{}\" already exists", body.username);
    return HttpResponse::Conflict().finish();
  }

  let LoginCredentials { username, password } = body.into_inner();

  log::info!("Creating user \"{}\"", username);
  let user = user::User::new(username, password);
  let token = State::generate_epicos_tokens();
  state.users.insert(token.clone(), user);

  state.write();
  HttpResponse::Ok().body(token)
}

#[actix_web::post("/login")]
async fn login(state: web::Data<AppState>, body: web::Json<LoginCredentials>) -> HttpResponse {
  let state = state.read().await;
  let LoginCredentials { username, password } = body.into_inner();

  let user = state.users.iter().find(|(_, user)| user.username == username && user.password == password);

  match user {
    Some((token, _)) => {
      log::info!("User \"{}\" logged in", username);
      HttpResponse::Ok().body(token.clone())
    },
    None => {
      log::info!("User \"{}\" failed to log in", username);
      HttpResponse::Unauthorized().finish()
    },
  }
}

pub fn routes() -> Scope {
  Scope::new("/users")
    .service(register)
    .service(login)
}