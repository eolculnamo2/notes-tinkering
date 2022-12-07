use std::fmt::Debug;
use rocket::serde::Deserialize;

use crate::{traits::object_mapping::ObjectMapping, dto::new_note_dto::NewNoteDto};

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewNote {
    pub title: String,
    pub body: String,
    pub category: String,
}

impl ObjectMapping<NewNoteDto> for NewNote {
    fn map_to(&self) -> NewNoteDto {
        NewNoteDto {
            title: self.title.clone(),
            body: self.body.clone(),
            category: self.category.clone(),
            user_fk: None,
        }        
    }
}
