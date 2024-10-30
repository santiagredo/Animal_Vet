use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Services {
    Table,
    ServiceId,
    Name,
    Duration,
    CreationDate,
    LatestUpdateDate,
    IsEnabled,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Services::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Services::ServiceId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Services::Name).text())
                    .col(ColumnDef::new(Services::Duration).integer())
                    .col(ColumnDef::new(Services::CreationDate).timestamp())
                    .col(ColumnDef::new(Services::IsEnabled).boolean())
                    .col(ColumnDef::new(Services::LatestUpdateDate).timestamp())
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Services::Table).cascade().to_owned())
            .await
            .unwrap();

        Ok(())
    }
}
