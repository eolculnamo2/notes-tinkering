use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserResponse {
   pub id: String,
   pub email: String,
   pub first_name: String,
   pub last_name: String,
}
