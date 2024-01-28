use super::user::User;
use super::report::Report;
use super::stream;
use crate::model::stream::ChunkSender;
use crate::AppState;

use std::{collections::HashMap, fs};
use std::sync::OnceLock;
use std::env;
use std::time::Duration;

use sha256::digest;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use tokio::sync::mpsc;
use actix_web::web::Bytes;
use tokio_stream::wrappers::ReceiverStream;

fn path() -> &'static str {
  static PATH: OnceLock<String> = OnceLock::new();
  PATH.get_or_init(|| {
    let path = match env::consts::OS {
      "windows" => env::var("APPDATA").unwrap(),
      "linux" => env::var("HOME").unwrap() + "/.config",
      _ => panic!("Unsupported OS")
    };

    format!("{}/{}", path, "ChildService.json")
  })
}

#[derive(Default, Serialize, Deserialize)]
pub struct State {
  pub users: HashMap<String, User>,
  pub reports: Vec<Report>,

  #[serde(skip)]
  pub listeners: Vec<(mpsc::Sender<Bytes>, String)>,
}

#[derive(Serialize)]
struct ToStrState<'a> {
  users: &'a HashMap<String, User>,
  reports: &'a Vec<Report>,
}

const PING_INTERVAL: Duration = Duration::from_secs(15);

pub trait PingLoop {
  fn start_ping_loop(&self);
}

impl PingLoop for AppState {
  fn start_ping_loop(&self) {
    let state = self.clone();

    tokio::spawn(async move {
      let mut interval = tokio::time::interval(PING_INTERVAL);
      loop {
        let mut state = state.write().await;
        state.listeners.retain(|(tx, _)| !tx.is_closed());

        drop(state);
        interval.tick().await;
      }
    }); 
  }
}

impl State {
  pub fn new() -> Self {
    match fs::read_to_string(path()) {
      Ok(file) => Self::from_str(&file),
      Err(_) => return State::default()
    }
  }

  fn from_str(file: &str) -> Self {
    match serde_json::from_str(file) {
      Ok(state) => state,
      Err(err) => panic!("Error while parsing state: {}", err)
    }
  }

  pub fn generate_epicos_tokens() -> String {
    let timestamp = Utc::now().timestamp_millis();
    let token = format!("{:x}", timestamp);

    log::debug!("Generated new token from: {}", token);
    digest(token)
  }

  pub fn write(&self) {
    let state = ToStrState {
      users: &self.users,
      reports: &self.reports,
    };

    log::debug!("Saving state to file...");
    let state = serde_json::to_string(&state).unwrap();
    if let Err(err) = fs::write(path(), state) {
      log::error!("Error while saving state to file: {}", err)
    }
  }

  pub fn new_stream(&mut self, user_id: String) -> stream::InfallibleStream<ReceiverStream<Bytes>> {
    let (tx, rx) = mpsc::channel(8);
    
    self.listeners.push((tx.clone(), user_id));
    tokio::spawn(async move {
      tx.send(Bytes::from("test")).await.unwrap();
    });

    stream::InfallibleStream::new(ReceiverStream::new(rx))
  }

  pub fn get_nicknames(&self) -> HashMap<&String, &String> {
    self.users.values().map(|user| (&user.uuid, &user.username)).collect::<HashMap<_, _>>()
  }

  pub async fn broadcast(&self, message: stream::StreamPayload<'_>) {
    let message = serde_json::to_string(&message).unwrap();
    let message = Bytes::from(message);
  
    log::debug!("Broadcasting message: {} to {} listeners", message.len(), self.listeners.len());
    for (listener, _) in &self.listeners {
      if listener.is_closed() {
        continue;
      }
    
      if let Err(err) = listener.send_chunk(message.clone()).await {
        log::error!("Error while sending message to listener: {}", err);
      }
    }
  }

  pub async fn broadcast_to(&self, user_id: &str, message: stream::StreamPayload<'_>) {
    let message = serde_json::to_string(&message).unwrap();
    let message = Bytes::from(message);
  
    log::debug!("Broadcasting message: {} to {}", message.len(), user_id);
    for (listener, id) in &self.listeners {
      if listener.is_closed() || id != user_id {
        continue;
      }
    
      if let Err(err) = listener.send_chunk(message.clone()).await {
        log::error!("Error while sending message to listener: {}", err);
      }
    }
  }
}
