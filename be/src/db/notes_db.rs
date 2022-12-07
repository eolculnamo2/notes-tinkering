use crate::{
    constants::db_constants::CONNECTION_PARAMS,
    db_entities::{note_entity::NoteEntity, note_list_entity::NotesListEntity},
};
use tokio_postgres::{Error, NoTls};

pub async fn save_new_note(new_note: NoteEntity) -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(CONNECTION_PARAMS, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.execute(
        "INSERT INTO notes (id, title, body, category, created, last_modified, user_fk) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[
            &new_note.id,
            &new_note.title,
            &new_note.body,
            &new_note.category,
            &new_note.created,
            &new_note.last_modified,
            &new_note.user_fk,
        ],
    ).await?;
    Ok(())
}

pub async fn get_notes_list(user_id: &str) -> Result<Vec<NotesListEntity>, Error> {
    let (client, connection) = tokio_postgres::connect(CONNECTION_PARAMS, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let result = client
        .query(
            "SELECT id, title, category, created, last_modified FROM notes WHERE user_fk = ($1)",
            &[&user_id],
        )
        .await?
        .iter()
        .map(|row| NotesListEntity {
            id: row.get(0),
            title: row.get(1),
            category: row.get(2),
            created: row.get(3),
            last_modified: row.get(4),
        })
        .collect::<Vec<NotesListEntity>>();
    Ok(result)
}

pub async fn get_note_by_id(user_id: &str, note_id: &str) -> Result<NoteEntity, Error> {
    let (client, connection) = tokio_postgres::connect(CONNECTION_PARAMS, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let row = client.query_one(
        "SELECT id, title, body, category, created, last_modified FROM notes WHERE user_fk = ($1) AND id = ($2)",
        &[&user_id, &note_id],
    ).await?;

    let result = NoteEntity {
        id: row.get(0),
        title: row.get(1),
        body: row.get(2),
        category: row.get(3),
        created: row.get(4),
        last_modified: row.get(5),
        user_fk: String::from(user_id),
    };
    Ok(result)
}

pub async fn delete_note_by_id(user_id: &str, note_id: &str) -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(CONNECTION_PARAMS, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
        .execute(
            "DELETE FROM notes WHERE user_fk = ($1) AND id = ($2)",
            &[&user_id, &note_id],
        )
        .await?;
    Ok(())
}
