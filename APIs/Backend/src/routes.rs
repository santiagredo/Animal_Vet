use actix_web::web;
use security::controller::{insert_reset_token, login, logout, update_user_password};

use crate::controller::{
    delete_pet, delete_user, insert_pet, insert_user, select_availability, select_pet_types,
    select_pets, select_services, select_user, update_pet, update_user,
};

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(insert_user)
            .service(select_user)
            .service(update_user)
            .service(delete_user),
    )
    .service(
        web::scope("/pets")
            .service(insert_pet)
            .service(select_pets)
            .service(update_pet)
            .service(delete_pet),
    )
    .service(web::scope("/pet_types").service(select_pet_types))
    .service(web::scope("/session").service(login).service(logout))
    .service(web::scope("/services").service(select_services))
    .service(web::scope("/availability").service(select_availability))
    .service(
        web::scope("/password_reset")
            .service(insert_reset_token)
            .service(update_user_password),
    );
}
