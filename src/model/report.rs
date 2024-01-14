use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Report {
  create_at: u64,
  subject: String,
  content: String,
  priority: Priority,
  user_uuid: String,
  eployee_uuid: String,
  uuid: String,
}

#[derive(Serialize, Deserialize)]
pub enum Priority {
  Minor,
  Major,
  Critical,
}