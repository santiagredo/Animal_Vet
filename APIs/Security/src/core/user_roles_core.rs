use models::entities::user_roles::Model;
use utils::{CodeMessage, Outcome};

use crate::data::UserRolesData;

pub struct UserRolesCore;

impl UserRolesCore {
    pub async fn select_role(user_role: Model) -> Outcome<Model, CodeMessage, CodeMessage> {
        match UserRolesData::select_role(user_role).await {
            Outcome::Failure(fail) => Outcome::Failure(fail),
            Outcome::Error(err) => Outcome::Error(err),
            Outcome::Success(val) => Outcome::Success(val),
        }
    }

    pub fn has_permission(user_permissions: i32, permission: i32) -> bool {
        (user_permissions & permission) != 0
    }
}
