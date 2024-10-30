use sea_orm_migration::prelude::*;

use crate::m20240706_045753_create_services::Services;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum UnavailableHours {
    Table,
    UnavailableHourId,
    ServiceId,
    Date,
    StartTime,
    EndTime,
    Reason,
    CreationDate,
    LatestUpdateDate,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UnavailableHours::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UnavailableHours::UnavailableHourId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UnavailableHours::ServiceId).integer())
                    .col(ColumnDef::new(UnavailableHours::Date).date())
                    .col(ColumnDef::new(UnavailableHours::StartTime).time())
                    .col(ColumnDef::new(UnavailableHours::EndTime).time())
                    .col(ColumnDef::new(UnavailableHours::Reason).text())
                    .col(ColumnDef::new(UnavailableHours::CreationDate).timestamp())
                    .col(ColumnDef::new(UnavailableHours::LatestUpdateDate).timestamp())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_unavailable_hours_service_id")
                    .from(UnavailableHours::Table, UnavailableHours::ServiceId)
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
                    .table(UnavailableHours::Table)
                    .cascade()
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }
}
