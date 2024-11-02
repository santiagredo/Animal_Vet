pub use sea_orm_migration::prelude::*;

mod m20240706_030902_create_user_roles;
mod m20240706_031656_create_user_role_events;
mod m20240706_032410_create_users;
mod m20240706_033314_create_user_events;
mod m20240706_034004_create_pet_types;
mod m20240706_034310_create_pet_type_events;
mod m20240706_034731_create_pets;
mod m20240706_035316_create_pet_events;
mod m20240706_045753_create_services;
mod m20240706_050053_create_service_events;
mod m20240706_050444_create_work_days;
mod m20240706_051229_create_work_day_events;
mod m20240706_052700_create_special_dates;
mod m20240706_054536_create_special_date_events;
mod m20240706_054837_create_unavailable_hours;
mod m20240706_055507_create_unavailable_hour_events;
mod m20240706_055925_create_appointments;
mod m20240706_060542_create_appointment_events;
mod m20240706_061017_create_medical_records;
mod m20240807_142445_create_sessions;
mod m20240807_143628_create_session_events;
mod m20241031_020452_create_password_reset;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240706_030902_create_user_roles::Migration),
            Box::new(m20240706_031656_create_user_role_events::Migration),
            Box::new(m20240706_032410_create_users::Migration),
            Box::new(m20240706_033314_create_user_events::Migration),
            Box::new(m20240706_034004_create_pet_types::Migration),
            Box::new(m20240706_034310_create_pet_type_events::Migration),
            Box::new(m20240706_034731_create_pets::Migration),
            Box::new(m20240706_035316_create_pet_events::Migration),
            Box::new(m20240706_045753_create_services::Migration),
            Box::new(m20240706_050053_create_service_events::Migration),
            Box::new(m20240706_050444_create_work_days::Migration),
            Box::new(m20240706_051229_create_work_day_events::Migration),
            Box::new(m20240706_052700_create_special_dates::Migration),
            Box::new(m20240706_054536_create_special_date_events::Migration),
            Box::new(m20240706_054837_create_unavailable_hours::Migration),
            Box::new(m20240706_055507_create_unavailable_hour_events::Migration),
            Box::new(m20240706_055925_create_appointments::Migration),
            Box::new(m20240706_060542_create_appointment_events::Migration),
            Box::new(m20240706_061017_create_medical_records::Migration),
            Box::new(m20240807_142445_create_sessions::Migration),
            Box::new(m20240807_143628_create_session_events::Migration),
            Box::new(m20241031_020452_create_password_reset::Migration),
        ]
    }
}
