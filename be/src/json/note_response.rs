use rocket::serde::Serialize;
use std::fmt::Debug;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NoteResponse {
    pub id: String,
    pub title: String,
    pub body: String,
    pub category: String,
    pub created: i32,
    pub last_modified: i32,
}
