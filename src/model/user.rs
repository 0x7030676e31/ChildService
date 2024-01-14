use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
  pub password: String,
  pub username: String,
  pub access_level: AccessLevel,
  pub uuid: String,
}

impl User {
  pub fn new(username: String, password: String) -> Self {
    Self {
      username,
      password,
      access_level: AccessLevel::User,
      uuid: uuid::Uuid::new_v4().to_string(),
    }
  }
}

#[derive(Serialize, Deserialize)]
pub enum AccessLevel {
  User,
  Employee,
  Admin,
}