use std::fmt::Debug;
use crate::json::user_response::UserResponse;
use crate::traits::object_mapping::ObjectMapping;

#[derive(Debug)]
pub struct UserEntity {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

impl ObjectMapping<UserResponse> for UserEntity {
    fn map_to(&self) -> UserResponse {
        UserResponse {
            id: self.id.clone(),
            email: self.email.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
        }
    }
}
