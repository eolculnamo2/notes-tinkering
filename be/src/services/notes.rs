use tokio_postgres::{Error};
use crate::json::note_response::NoteResponse;
use crate::traits::object_mapping::ObjectMapping;
use crate::dto::new_note_dto::NewNoteDto;
use crate::json::notes_list_item::NotesListItem;
use crate::db::notes_db::{save_new_note, get_notes_list, get_note_by_id, delete_note_by_id};

pub async fn create_note(new_note: NewNoteDto) -> Result<(), Error> {
    let entity = new_note.map_to();
    save_new_note(entity).await
}

pub async fn get_notes_list_by_user(user_id: &str) -> Result<Vec<NotesListItem>, Error> {
    let list_items = get_notes_list(user_id).await?
        .iter()
        .map(|item| item.map_to())
        .collect::<Vec<NotesListItem>>();
    Ok(list_items)
}

pub async fn get_full_note(user_id: &str, note_id: &str) -> Result<NoteResponse, Error> {
    let note = get_note_by_id(user_id, note_id).await?.map_to();
    Ok(note)
}

pub async fn delete_note(user_id: &str, note_id: &str) -> Result<(), Error> {
    delete_note_by_id(user_id, note_id).await
}
