use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum UserEvents {
    Table,
    UserEventId,
    UserId,
    Details,
    CreationDate,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserEvents::Table)
                    .col(
                        ColumnDef::new(UserEvents::UserEventId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserEvents::UserId).integer())
                    .col(ColumnDef::new(UserEvents::Details).text())
                    .col(ColumnDef::new(UserEvents::CreationDate).timestamp())
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserEvents::Table).cascade().to_owned())
            .await
            .unwrap();

        Ok(())
    }
}
