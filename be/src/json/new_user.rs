use std::fmt::Debug;
use crate::dto::new_user_dto::NewUserDto;
use crate::traits::object_mapping::ObjectMapping;
use rocket::serde::Deserialize;
use uuid::Uuid;
use bcrypt;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

impl ObjectMapping<NewUserDto> for NewUser {
    fn map_to(&self) -> NewUserDto {
        NewUserDto {
            id: Uuid::new_v4(),
            email: self.email.clone(),
            password: bcrypt::hash(self.password.clone(), 10).unwrap(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
        }
    }
}
