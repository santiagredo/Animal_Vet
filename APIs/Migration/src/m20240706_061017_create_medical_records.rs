use sea_orm_migration::prelude::*;

use crate::{m20240706_032410_create_users::Users, m20240706_034731_create_pets::Pets};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum MedicalRecords {
    Table,
    MedicalRecordId,
    PetId,
    CreationDate,
    Comments,
    CreatedByUserId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MedicalRecords::Table)
                    .col(
                        ColumnDef::new(MedicalRecords::MedicalRecordId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MedicalRecords::PetId).integer())
                    .col(ColumnDef::new(MedicalRecords::CreationDate).timestamp())
                    .col(ColumnDef::new(MedicalRecords::Comments).text())
                    .col(ColumnDef::new(MedicalRecords::CreatedByUserId).integer())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_medical_records_pets")
                    .from(MedicalRecords::Table, MedicalRecords::PetId)
                    .to(Pets::Table, Pets::PetId)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_medical_records_users")
                    .from(MedicalRecords::Table, MedicalRecords::CreatedByUserId)
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
                    .table(MedicalRecords::Table)
                    .cascade()
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }
}
