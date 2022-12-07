use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

type TimeStamp = i64;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NotesListItem {
    pub id: String,
    pub title: String,
    pub category: String,
    pub created: TimeStamp,
    pub last_modified: TimeStamp,
}
