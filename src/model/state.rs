use super::user::User;
use super::thread::Thread;
use super::report::Report;
use super::stream;
use crate::{ArcLock, AppState};

use std::collections::HashMap;
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

#[derive(Default)]
pub struct State {
  pub users: HashMap<String, ArcLock<User>>,
  pub threads: Vec<ArcLock<Thread>>,
  pub reports: Vec<ArcLock<Report>>,
  pub listeners: Vec<(mpsc::Sender<Bytes>, String)>,
}

#[derive(Deserialize)]
struct FromStrState {
  users: HashMap<String, User>,
  threads: Vec<Thread>,
  reports: Vec<Report>,
}

#[derive(Serialize)]
struct ToStrState<'a> {
  users: HashMap<String, &'a User>,
  threads: Vec<&'a Thread>,
  reports: Vec<&'a Report>,
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
    match std::fs::read_to_string(path()) {
      Ok(file) => Self::from_str(&file),
      Err(_) => return State::default()
    }
  }

  fn from_str(file: &str) -> Self {
    let state: FromStrState = serde_json::from_str(file).unwrap();
    log::info!("Loaded state from file");

    State {
      users: state.users.into_iter().map(|(k, v)| (k, ArcLock(v))).collect(),
      threads: state.threads.into_iter().map(|v| ArcLock(v)).collect(),
      reports: state.reports.into_iter().map(|v| ArcLock(v)).collect(),
      listeners: Vec::new(),
    }
  }

  pub fn generate_epicos_tokens() -> String {
    let timestamp = Utc::now().timestamp_millis();
    let token = format!("{:x}", timestamp);

    log::info!("Generated new token from: {}", token);
    digest(token)
  }

  pub async fn write(&self) {
  
  }

  pub fn new_stream(&mut self, user_id: String) -> stream::InfallibleStream<ReceiverStream<Bytes>> {
    let (tx, rx) = mpsc::channel(8);
    
    self.listeners.push((tx.clone(), user_id));
    tokio::spawn(async move {
      tx.send(Bytes::from("test")).await.unwrap();
    });

    stream::InfallibleStream::new(ReceiverStream::new(rx))
  } 

  pub async fn broadcast(&self, message: stream::StreamPayload<'_>) {
    let message = serde_json::to_string(&message).unwrap();
    let message = Bytes::from(message);
  
    for (listener, _) in &self.listeners {
      if listener.is_closed() {
        continue;
      }
    
      if let Err(err) = listener.send(message.clone()).await {
        log::error!("Error while sending message to listener: {}", err);
      }
    }
  }

  pub async fn broadcast_to(&self, user_id: &str, message: stream::StreamPayload<'_>) {
    let message = serde_json::to_string(&message).unwrap();
    let message = Bytes::from(message);
  
    for (listener, id) in &self.listeners {
      if listener.is_closed() || id != user_id {
        continue;
      }
    
      if let Err(err) = listener.send(message.clone()).await {
        log::error!("Error while sending message to listener: {}", err);
      }
    }
  }
}
