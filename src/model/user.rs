use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
  pub password: String,
  pub username: String,
  pub access_level: AccessLevel,
  pub uuid: String,
}

#[derive(Serialize)]
pub struct ToStrUser<'a> {
  pub username: &'a str,
  pub access_level: &'a AccessLevel,
  pub uuid: &'a str,
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

  pub fn web_user(&self) -> ToStrUser {
    ToStrUser {
      username: &self.username,
      access_level: &self.access_level,
      uuid: &self.uuid,
    }
  }
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub enum AccessLevel {
  User,
  Employee,
  Admin,
}