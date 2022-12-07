use std::fmt::Debug;
use rocket::serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

