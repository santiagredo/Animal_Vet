use sea_orm_migration::prelude::*;

use crate::m20240706_045753_create_services::Services;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum SpecialDates {
    Table,
    SpecialDateId,
    ServiceId,
    Date,
    IsWorkingDate,
    OpenTime,
    CloseTime,
    LunchFromTime,
    LunchToTime,
    CreationDate,
    LatestUpdateDate,
    Reason
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SpecialDates::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SpecialDates::SpecialDateId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SpecialDates::ServiceId).integer())
                    .col(ColumnDef::new(SpecialDates::Date).date())
                    .col(ColumnDef::new(SpecialDates::IsWorkingDate).boolean())
                    .col(ColumnDef::new(SpecialDates::OpenTime).time())
                    .col(ColumnDef::new(SpecialDates::CloseTime).time())
                    .col(ColumnDef::new(SpecialDates::LunchFromTime).time())
                    .col(ColumnDef::new(SpecialDates::LunchToTime).time())
                    .col(ColumnDef::new(SpecialDates::CreationDate).timestamp())
                    .col(ColumnDef::new(SpecialDates::LatestUpdateDate).timestamp())
                    .col(ColumnDef::new(SpecialDates::Reason).text())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_special_date_service_id")
                    .from(SpecialDates::Table, SpecialDates::ServiceId)
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
                    .table(SpecialDates::Table)
                    .cascade()
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }
}
