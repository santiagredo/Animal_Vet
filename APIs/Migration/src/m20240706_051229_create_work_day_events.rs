use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum WorkDayEvents {
    Table,
    WorkDayEventId,
    WorkDayId,
    Details,
    CreationDate,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorkDayEvents::Table)
                    .col(
                        ColumnDef::new(WorkDayEvents::WorkDayEventId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WorkDayEvents::WorkDayId).integer())
                    .col(ColumnDef::new(WorkDayEvents::Details).text())
                    .col(ColumnDef::new(WorkDayEvents::CreationDate).timestamp())
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
                    .table(WorkDayEvents::Table)
                    .cascade()
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }
}
