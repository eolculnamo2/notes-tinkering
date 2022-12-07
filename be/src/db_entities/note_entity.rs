use crate::{traits::object_mapping::ObjectMapping, json::note_response::NoteResponse};

pub struct NoteEntity {
    pub id: String,
    pub title: String,
    pub body: String,
    pub category: String,
    pub created: i32,
    pub last_modified: i32,
    pub user_fk: String,
}

impl ObjectMapping<NoteResponse> for NoteEntity {
    fn map_to(&self) -> NoteResponse {
        NoteResponse {
            id: self.id.clone(),
            title: self.title.clone(),
            body: self.body.clone(),
            category: self.category.clone(),
            created: self.created,
            last_modified: self.last_modified,
        }
    }
}

