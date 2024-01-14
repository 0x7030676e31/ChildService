use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Thread {
  messages: Vec<Message>,
  is_resolved: bool,
  report_uuid: String,
  uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
  author_uuid: String,
  content: String,
  create_at: u64,
}
