use crate::entities::user_entity::User;
use crate::infra::models::user::user_model::UserModel;
use mysql::Result;

pub struct EntityModelMapper {}

impl EntityModelMapper {
    pub fn new() -> Self {
        EntityModelMapper {}
    }

    pub fn map_to_entity(&self, model: &UserModel) -> User {
        User::new(model.id, model.name.clone())
    }

    pub fn map_to_model(&self, data: Result<mysql::Row>) -> UserModel {
        let row = data.unwrap();

        UserModel {
            id: row.get("id").expect("id not found"),
            name: row.get("name").expect("name not found"),
        }
    }
}
