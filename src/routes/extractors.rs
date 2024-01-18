use crate::AppState;

use std::pin::Pin;

use actix_web::{FromRequest, Error, HttpRequest, error, web};
use actix_web::dev::Payload;
use futures::Future;

pub struct UserGuard(pub String);

impl FromRequest for UserGuard {
  type Error = Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    let app_state = req.app_data::<web::Data<AppState>>().unwrap().clone();
    let header = req.headers().get("Authorization").map(|header| header.to_owned());

    let path = req.path().to_owned();
    Box::pin(async move {
      let header = match header {
        Some(header) => header,
        None => {
          log::warn!("Missing authorization header. Request path: {}", path);
          return Err(error::ErrorUnauthorized(""))
        },
      };

      let header = match header.to_str() {
        Ok(header) => header,
        Err(_) => {
          log::warn!("Invalid authorization header. Request path: {}", path);
          return Err(error::ErrorUnauthorized(""))
        },
      };

      
      let app_state = app_state.read().await;
      if app_state.users.contains_key(header) {
        Ok(UserGuard(header.to_owned()))
      } else {
        log::warn!("Invalid authorization token. Request path: {}", path);
        Err(error::ErrorForbidden(""))
      }
    })
  }
}
