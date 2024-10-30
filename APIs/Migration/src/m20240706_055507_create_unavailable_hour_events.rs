use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum UnavailableHourEvents {
    Table,
    UnavailableHourEventId,
    UnavailableHourId,
    Details,
    CreationDate,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UnavailableHourEvents::Table)
                    .col(
                        ColumnDef::new(UnavailableHourEvents::UnavailableHourEventId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UnavailableHourEvents::UnavailableHourId).integer())
                    .col(ColumnDef::new(UnavailableHourEvents::Details).text())
                    .col(
                        ColumnDef::new(UnavailableHourEvents::CreationDate)
                            .timestamp(),
                    )
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
                    .table(UnavailableHourEvents::Table)
                    .cascade()
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }
}
