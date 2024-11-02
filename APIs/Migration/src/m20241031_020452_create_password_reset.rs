use sea_orm_migration::prelude::*;

use crate::m20240706_032410_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum PasswordReset {
    Table,
    Id,
    UserId,
    CreationDate,
    IsEnabled,
    UpdateDate,
    Token,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PasswordReset::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PasswordReset::Id)
                            .primary_key()
                            .integer()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(PasswordReset::UserId).integer())
                    .col(ColumnDef::new(PasswordReset::CreationDate).timestamp())
                    .col(ColumnDef::new(PasswordReset::IsEnabled).boolean())
                    .col(ColumnDef::new(PasswordReset::UpdateDate).timestamp())
                    .col(ColumnDef::new(PasswordReset::Token).text())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user_id")
                    .from(PasswordReset::Table, PasswordReset::UserId)
                    .to(Users::Table, Users::UserId)
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
                    .table(PasswordReset::Table)
                    .cascade()
                    .to_owned(),
            )
            .await
    }
}
