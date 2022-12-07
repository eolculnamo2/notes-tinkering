use crate::{db_entities::note_entity::NoteEntity, traits::object_mapping::ObjectMapping};
use chrono::offset::Utc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct NewNoteDto {
    pub title: String,
    pub body: String,
    pub category: String,
    pub user_fk: Option<String>,
}

impl ObjectMapping<NoteEntity> for NewNoteDto {
    fn map_to(&self) -> NoteEntity {
        NoteEntity {
            id: Uuid::new_v4().to_string(),
            title: self.title.clone(),
            body: self.body.clone(),
            category: self.category.clone(),
            created: Utc::now().timestamp() as i32,
            last_modified: Utc::now().timestamp() as i32,
            user_fk: self
                .user_fk
                .clone()
                .take()
                .unwrap_or_else(|| panic!("User foreign key is required")),
        }
    }
}
