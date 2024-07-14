use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct User {
	pub sub: String,
    pub name: String,
    pub givenname: String,
    pub familyname: String,
    pub preferredusername: String,
    pub email: String,
    pub picture: String,
}