use tokio_postgres::{Error, NoTls};

use crate::db_entities::user_entity::UserEntity;
use crate::constants::db_constants::CONNECTION_PARAMS;

pub async fn save_new_user(new_user: UserEntity) -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(CONNECTION_PARAMS, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.execute(
        "INSERT INTO users (id, email, password, first_name, last_name) VALUES ($1, $2, $3, $4, $5)",
        &[
            &new_user.id,
            &new_user.email,
            &new_user.password,
            &new_user.first_name,
            &new_user.last_name,
        ],
    ).await?;
    Ok(())
}

pub async fn find_user_by_email(email: String) -> Result<UserEntity, Error> {
    let (client, connection) = tokio_postgres::connect(CONNECTION_PARAMS, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let row = client.query_one("SELECT * FROM users WHERE email = $1", &[&email]).await?;
    Ok(UserEntity {
        id: row.get("id"),
        email: row.get("email"),
        password: row.get("password"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
    })
}
