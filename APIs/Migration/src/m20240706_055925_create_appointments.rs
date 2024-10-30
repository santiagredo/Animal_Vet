use sea_orm_migration::prelude::*;

use crate::{
    m20240706_032410_create_users::Users, m20240706_034731_create_pets::Pets,
    m20240706_045753_create_services::Services,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Appointments {
    Table,
    AppointmentId,
    UserId,
    PetId,
    Date,
    ServiceId,
    IsCanceled,
    CancellationDate,
    CreationDate,
    LatestUpdateDate,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Appointments::Table)
                    .col(
                        ColumnDef::new(Appointments::AppointmentId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Appointments::UserId).integer())
                    .col(ColumnDef::new(Appointments::PetId).integer())
                    .col(ColumnDef::new(Appointments::Date).timestamp())
                    .col(ColumnDef::new(Appointments::ServiceId).integer())
                    .col(ColumnDef::new(Appointments::IsCanceled).boolean())
                    .col(ColumnDef::new(Appointments::CancellationDate).timestamp())
                    .col(ColumnDef::new(Appointments::CreationDate).timestamp())
                    .col(ColumnDef::new(Appointments::LatestUpdateDate).timestamp())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_appointments_users")
                    .from(Appointments::Table, Appointments::UserId)
                    .to(Users::Table, Users::UserId)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_appointments_pets")
                    .from(Appointments::Table, Appointments::PetId)
                    .to(Pets::Table, Pets::PetId)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_appointments_services")
                    .from(Appointments::Table, Appointments::ServiceId)
                    .to(Services::Table, Services::ServiceId)
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Appointments::Table)
                    .cascade()
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }
}
