use uuid::Uuid;
use std::fmt::Debug;
use crate::db_entities::user_entity::UserEntity;
use crate::traits::object_mapping::ObjectMapping;

#[derive(Debug)]
pub struct NewUserDto {
   pub id: Uuid,
   pub email: String,
   pub first_name: String,
   pub last_name: String,
   pub password: String,
}

impl ObjectMapping<UserEntity> for NewUserDto {
    fn map_to(&self) -> UserEntity {
        UserEntity {
            id: self.id.to_string(),
            email: self.email.clone(),
            password: bcrypt::hash(self.password.clone(), 10).unwrap(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
        }
    }
}

