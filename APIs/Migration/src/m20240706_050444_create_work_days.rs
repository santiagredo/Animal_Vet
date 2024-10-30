use sea_orm_migration::prelude::*;

use crate::m20240706_045753_create_services::Services;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Days {
    Table,
    DayId,
    Name,
}

#[derive(DeriveIden)]
pub enum WorkDays {
    Table,
    WorkDayId,
    ServiceId,
    DayId,
    IsEnabled,
    CreationDate,
    LatestUpdateDate,
    OpenTime,
    CloseTime,
    LunchFromTime,
    LunchToTime,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Days::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Days::DayId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Days::Name).text())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                    .table(WorkDays::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WorkDays::WorkDayId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WorkDays::ServiceId).integer())
                    .col(ColumnDef::new(WorkDays::DayId).integer())
                    .col(ColumnDef::new(WorkDays::IsEnabled).boolean())
                    .col(ColumnDef::new(WorkDays::CreationDate).timestamp())
                    .col(ColumnDef::new(WorkDays::LatestUpdateDate).timestamp())
                    .col(ColumnDef::new(WorkDays::OpenTime).time())
                    .col(ColumnDef::new(WorkDays::CloseTime).time())
                    .col(ColumnDef::new(WorkDays::LunchFromTime).time())
                    .col(ColumnDef::new(WorkDays::LunchToTime).time())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_service_id")
                    .from(WorkDays::Table, WorkDays::ServiceId)
                    .to(Services::Table, Services::ServiceId)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_day_id")
                    .from(WorkDays::Table, WorkDays::DayId)
                    .to(Days::Table, WorkDays::DayId)
                    .to_owned(),
            )
            .await
            .unwrap();

        let insert_days_of_week = Query::insert()
            .into_table(Days::Table)
            .columns([Days::Name])
            .values_panic(["Sunday".into()])
            .values_panic(["Monday".into()])
            .values_panic(["Tuesday".into()])
            .values_panic(["Wednesday".into()])
            .values_panic(["Thursday".into()])
            .values_panic(["Friday".into()])
            .values_panic(["Saturday".into()])
            .to_owned();

        manager.exec_stmt(insert_days_of_week).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WorkDays::Table).cascade().to_owned())
            .await
            .unwrap();

        manager
            .drop_table(Table::drop().table(Days::Table).cascade().to_owned())
            .await
            .unwrap();

        Ok(())
    }
}
