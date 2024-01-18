use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Report {
  pub created_at: u64,
  pub report_details: ReportDetails,
  pub priority: Priority,
  pub user_uuid: String,
  pub employee_uuid: String,
  pub messages: Vec<Message>,
  pub is_resolved: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ReportDetails {
  pub subject: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
  pub created_at: u64,
  pub content: String,
  pub author_uuid: String,
}

#[derive(Serialize, Deserialize)]
pub enum Priority {
  Minor,
  Major,
  Critical,
}