use crate::model::stream::{InfallibleStream, StreamPayload};
use crate::model::user::AccessLevel;
use super::extractors::UserGuard;
use crate::AppState;

use actix_web::web::Bytes;
use actix_web::{get, web, Responder};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

#[get("/stream")]
pub async fn stream(state: web::Data<AppState>, token: UserGuard) -> impl Responder {
  let mut state = state.write().await;
  
  let (tx, rx) = mpsc::channel(4);
  let rx = ReceiverStream::new(rx);

  let user = state.users.get(&token.0).unwrap();
  let payload = if user.access_level == AccessLevel::Admin {
    StreamPayload::ReadyAdmin {
      id: &user.uuid,
      users: state.users.iter().map(|(_, user)| user.web_user()).collect::<Vec<_>>(),
      reports: &state.reports,
    }
  } else {
    let reports = state.reports.iter().filter(|report| report.user_uuid == user.uuid).collect::<Vec<_>>();
    StreamPayload::Ready {
      user: user.web_user(),
      reports,
      nicknames: state.get_nicknames(),
    }
  };

  let payload = serde_json::to_string(&payload).unwrap();
  let payload = Bytes::from(payload);
  state.listeners.push((tx.clone(), token.0));
  
  tokio::spawn(async move {
    if let Err(err) = tx.send(payload).await {
      log::error!("Error sending payload to stream: {}", err);
    }
  });
  
  InfallibleStream::new(rx)
}