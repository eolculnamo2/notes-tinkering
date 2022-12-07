#[macro_use]
extern crate rocket;

use std::io::Cursor;

use json::note_response;
use rocket::fairing::AdHoc;
use rocket::response::Responder;
use rocket::response;
use rocket::Request;
use rocket::{http::Status, Response};
use rocket::serde::json::Json;
use services::notes::delete_note;
use services::notes::get_full_note;
use tokio_postgres::Error;
use rocket::http::Method;
use rocket::http::Header;
use rocket::fairing::{Fairing, Info, Kind};

mod constants;
mod db;
mod db_entities;
mod dto;
mod json;
mod services;
mod traits;

use json::credentials::Credentials;
use json::new_user::NewUser;
use json::user_response::UserResponse;
use services::notes::{self, get_notes_list_by_user};
use traits::object_mapping::ObjectMapping;

#[post("/new-user", format = "json", data = "<new_user>")]
async fn register_user(new_user: Json<NewUser>) -> Status {
    match services::register::make(new_user.into_inner()).await {
        Err(_) => Status::Conflict, // or some error code
        Ok(_) => Status::InternalServerError,
    }
}

#[post("/login", format = "json", data = "<credentials>")]
pub async fn login(credentials: Json<Credentials>) -> Json<UserResponse> {
    let user = services::login::login(credentials.into_inner()).await;
    let response = user.unwrap().map_to();
    Json(response)
}

#[post("/users/<user_id>/notes", format = "json", data = "<new_note>")]
async fn create_note(user_id: &str, new_note: Json<json::new_note::NewNote>) -> Status {
    let mut dto = new_note.into_inner().map_to();
    dto.user_fk = Some(String::from(user_id));
    match notes::create_note(dto).await {
        Ok(_) => Status::Created, 
        Err(e) => {
            println!("{}", e);
            Status::InternalServerError
        }
    }
}

#[get("/users/<user_id>/notes")]
async fn get_notes_list(user_id: &str) -> Json<Vec<json::notes_list_item::NotesListItem>> {
    let list_items = get_notes_list_by_user(user_id).await;
    match list_items {
        Ok(items) => Json(items),
        Err(e) => panic!("Error: {}", e),
    }
}

#[get("/users/<user_id>/notes/<note_id>")]
async fn get_note(user_id: &str, note_id: &str) -> Json<json::note_response::NoteResponse> {
    let note_response = get_full_note(user_id, note_id).await;

    match note_response {
        Ok(note) => Json(note),
        Err(e) => panic!("{}", e),
    }
}

#[delete("/users/<user_id>/notes/<note_id>")]
async fn handle_delete_note(user_id: &str, note_id: &str) -> Status {
    match delete_note(user_id, note_id).await {
       Ok(_)  => Status::Ok,
       Err(e) => {
           println!("{}", e);
           Status::InternalServerError
       }
    } 
}

// tinkering
struct Ping {
    p: String,
    i: String,
    ng: String,
}
impl<'r> Responder<'r, 'static> for Ping {
     fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let r = Response::build()
            .status(Status::ImATeapot)
            .sized_body(self.ng.len(), Cursor::new(self.ng))
            .finalize();
         Ok(r)
    }
}
#[get("/ping")]
fn ping() -> Ping {
   Ping {
       p: String::from("p"),
       i: String::from("i"),
       ng: String::from("ng"),
   } 
}

#[options("/<_..>")]
fn opts() {
    println!("options...")
}


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "OPTIONS, POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
        rocket::build()
        .attach(CORS)
        .attach(AdHoc::on_liftoff("LIFT OFF!", |_| Box::pin (async move{
            println!("YOU-STAWN WE HAVE LIVE OFFFF");
        })))
        .attach(AdHoc::on_request("Request made", |req, _| Box::pin (async move{
            println!("Request made!");
        })))
        .mount("/", routes![opts, ping, handle_delete_note, get_note, get_notes_list, create_note, register_user, login])
}
