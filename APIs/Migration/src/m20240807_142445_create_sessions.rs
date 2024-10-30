use sea_orm_migration::prelude::*;

use crate::m20240706_032410_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Sessions {
    Table,
    SessionId,
    UserId,
    SessionUuid,
    IsEnabled,
    CreationDate,
    LatestUpdateDate,
    ClosingDate,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .col(
                        ColumnDef::new(Sessions::SessionId)
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Sessions::UserId).integer())
                    .col(ColumnDef::new(Sessions::SessionUuid).uuid())
                    .col(ColumnDef::new(Sessions::IsEnabled).boolean())
                    .col(ColumnDef::new(Sessions::CreationDate).timestamp())
                    .col(ColumnDef::new(Sessions::LatestUpdateDate).timestamp())
                    .col(ColumnDef::new(Sessions::ClosingDate).timestamp())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user_id")
                    .from(Sessions::Table, Sessions::UserId)
                    .to(Users::Table, Users::UserId)
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sessions::Table).cascade().to_owned())
            .await
            .unwrap();

        Ok(())
    }
}
