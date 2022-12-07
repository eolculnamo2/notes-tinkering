use crate::{traits::object_mapping::ObjectMapping, json::notes_list_item::NotesListItem};

type TimeStamp = i32;

pub struct NotesListEntity {
    pub id: String,
    pub title: String,
    pub category: String,
    pub created: TimeStamp,
    pub last_modified: TimeStamp,
}

impl ObjectMapping<NotesListItem> for NotesListEntity {
    fn map_to(&self) -> NotesListItem {
        NotesListItem {
            id: self.id.clone(),
            title: self.title.clone(),
            category: self.category.clone(),
            created: self.created as i64,
            last_modified: self.last_modified as i64,
        }
    }
}
