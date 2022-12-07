use crate::db::user_db::save_new_user;
use crate::dto::new_user_dto::NewUserDto;
use crate::json::new_user::NewUser;
use crate::traits::object_mapping::ObjectMapping;

pub async fn make(new_user: NewUser) -> Result<NewUserDto, String> {
    let dto = new_user.map_to();
    match save_new_user(dto.map_to()).await {
        Err(_) => Err(String::from("Failed to save user")),
        Ok(_) => Ok(dto),
    }
}
