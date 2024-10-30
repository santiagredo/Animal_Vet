use sea_orm_migration::prelude::*;

use crate::{m20240706_032410_create_users::Users, m20240706_034004_create_pet_types::PetTypes};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Pets {
    Table,
    PetId,
    Name,
    CreationDate,
    LatestUpdateDate,
    PetTypeId,
    UserId,
    AdditionalComments,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Pets::Table)
                    .col(
                        ColumnDef::new(Pets::PetId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Pets::PetTypeId).integer())
                    .col(ColumnDef::new(Pets::Name).text())
                    .col(ColumnDef::new(Pets::CreationDate).timestamp())
                    .col(ColumnDef::new(Pets::LatestUpdateDate).timestamp())
                    .col(ColumnDef::new(Pets::UserId).integer())
                    .col(ColumnDef::new(Pets::AdditionalComments).text())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_pet_type")
                    .from(Pets::Table, Pets::PetTypeId)
                    .to(PetTypes::Table, PetTypes::PetTypeId)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user_id")
                    .from(Pets::Table, Pets::UserId)
                    .to(Users::Table, Users::UserId)
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_pet_type")
                    .table(Pets::Table)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_user_id")
                    .table(Pets::Table)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .drop_table(Table::drop().table(Pets::Table).cascade().to_owned())
            .await
            .unwrap();

        Ok(())
    }
}
