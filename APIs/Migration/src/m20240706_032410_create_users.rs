use sea_orm_migration::prelude::*;

use crate::m20240706_030902_create_user_roles::UserRoles;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Users {
    Table,
    UserId,
    Name,
    Email,
    Password,
    PhoneNumber,
    DocumentId,
    Role,
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
                    .table(Users::Table)
                    .col(
                        ColumnDef::new(Users::UserId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Name).text())
                    .col(ColumnDef::new(Users::Email).text())
                    .col(ColumnDef::new(Users::Password).text())
                    .col(ColumnDef::new(Users::PhoneNumber).text())
                    .col(ColumnDef::new(Users::DocumentId).text())
                    .col(ColumnDef::new(Users::Role).integer())
                    .col(ColumnDef::new(Users::CreationDate).timestamp())
                    .col(ColumnDef::new(Users::LatestUpdateDate).timestamp())
                    .col(ColumnDef::new(Users::IsEnabled).boolean())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user_roles")
                    .from(Users::Table, Users::Role)
                    .to(UserRoles::Table, UserRoles::UserRoleId)
                    .to_owned(),
            )
            .await
            .unwrap();

        let insert_initial_admin = Query::insert()
            .into_table(Users::Table)
            .columns([
                Users::Name,
                Users::Email,
                Users::Password,
                Users::PhoneNumber,
                Users::DocumentId,
                Users::Role,
                Users::CreationDate,
                Users::IsEnabled,
            ])
            .values_panic([
                "".into(),
                "admin+animalvet@proton.me".into(),
                "$2b$12$5MfqmIXm311yppwNYazESur3LV9bZDK7Xd.Ra1F8CNUHT6x71DlfK".into(),
                "".into(),
                "".into(),
                1.into(),
                Func::cast_as(
                    "2024-06-21 00:00:00-5",
                    Alias::new("TIMESTAMP"),
                )
                .into(),
                true.into(),
            ])
            .to_owned();

        manager.exec_stmt(insert_initial_admin).await?;

        let insert_initial_staff = Query::insert()
            .into_table(Users::Table)
            .columns([
                Users::Name,
                Users::Email,
                Users::Password,
                Users::PhoneNumber,
                Users::DocumentId,
                Users::Role,
                Users::CreationDate,
                Users::IsEnabled,
            ])
            .values_panic([
                "".into(),
                "tests_staff@tests.com".into(),
                "$2b$12$UmqXKXfW1zi96YMrf7nEMeOpGCFbk8MJ.NsVxGHMn5oz6mHSq9mrS".into(),
                "".into(),
                "".into(),
                2.into(),
                Func::cast_as(
                    "2024-06-21 00:00:00-5",
                    Alias::new("TIMESTAMP"),
                )
                .into(),
                true.into(),
            ])
            .to_owned();

        manager.exec_stmt(insert_initial_staff).await?;

        let insert_tests_user = Query::insert()
            .into_table(Users::Table)
            .columns([
                Users::Name,
                Users::Email,
                Users::Password,
                Users::PhoneNumber,
                Users::DocumentId,
                Users::Role,
                Users::CreationDate,
                Users::IsEnabled,
            ])
            .values_panic([
                "".into(),
                "tests_user@tests.com".into(),
                "$2b$12$UmqXKXfW1zi96YMrf7nEMeOpGCFbk8MJ.NsVxGHMn5oz6mHSq9mrS".into(),
                "".into(),
                "".into(),
                3.into(),
                Func::cast_as(
                    "2024-08-10 00:00:00-5",
                    Alias::new("TIMESTAMP"),
                )
                .into(),
                true.into(),
            ])
            .to_owned();

        manager.exec_stmt(insert_tests_user).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).cascade().to_owned())
            .await
            .unwrap();

        Ok(())
    }
}
