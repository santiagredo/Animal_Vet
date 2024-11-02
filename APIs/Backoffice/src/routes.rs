use actix_web::web;
use security::controller::{insert_reset_token, login, logout, update_user_password};

use crate::controller::{
    delete_appointment, delete_pet, delete_service, delete_special_date, delete_unavailable_hours,
    delete_user, delete_work_day, insert_appointment, insert_medical_record, insert_pet,
    insert_service, insert_special_date, insert_unavailable_hours, insert_user, insert_work_day,
    select_appointments, select_availability, select_days, select_medical_records,
    select_pet_types, select_pets, select_services, select_special_dates, select_unavailable_hours,
    select_user, select_work_days, update_appointment, update_pet, update_service,
    update_special_date, update_unavailble_hours, update_user, update_work_day,
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
    .service(
        web::scope("/services")
            .service(insert_service)
            .service(select_services)
            .service(update_service)
            .service(delete_service),
    )
    .service(
        web::scope("/work_days")
            .service(insert_work_day)
            .service(select_work_days)
            .service(update_work_day)
            .service(delete_work_day),
    )
    .service(
        web::scope("/special_dates")
            .service(insert_special_date)
            .service(select_special_dates)
            .service(update_special_date)
            .service(delete_special_date),
    )
    .service(
        web::scope("/unavailable_hours")
            .service(insert_unavailable_hours)
            .service(select_unavailable_hours)
            .service(update_unavailble_hours)
            .service(delete_unavailable_hours),
    )
    .service(
        web::scope("/appointments")
            .service(insert_appointment)
            .service(select_appointments)
            .service(update_appointment)
            .service(delete_appointment),
    )
    .service(
        web::scope("medical_records")
            .service(insert_medical_record)
            .service(select_medical_records),
    )
    .service(web::scope("/availability").service(select_availability))
    .service(web::scope("/days").service(select_days))
    .service(
        web::scope("/password_reset")
            .service(insert_reset_token)
            .service(update_user_password),
    );
}
