use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveMigrationName, DeriveIden)]
pub enum PetTypes {
    Table,
    PetTypeId,
    Name,
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
                    .table(PetTypes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PetTypes::PetTypeId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PetTypes::Name).text())
                    .col(ColumnDef::new(PetTypes::CreationDate).timestamp())
                    .col(ColumnDef::new(PetTypes::LatestUpdateDate).timestamp())
                    .col(ColumnDef::new(PetTypes::IsEnabled).boolean())
                    .to_owned(),
            )
            .await
            .unwrap();

        let insert_initial_pet_types = Query::insert()
            .into_table(PetTypes::Table)
            .columns([PetTypes::Name, PetTypes::CreationDate, PetTypes::IsEnabled])
            .values_panic([
                "dog".into(),
                Func::cast_as(
                    "2024-06-21 00:00:00-5",
                    Alias::new("TIMESTAMP"),
                )
                .into(),
                true.into(),
            ])
            .values_panic([
                "cat".into(),
                Func::cast_as(
                    "2024-06-21 00:00:00-5",
                    Alias::new("TIMESTAMP"),
                )
                .into(),
                true.into(),
            ])
            .values_panic([
                "fish".into(),
                Func::cast_as(
                    "2024-06-21 00:00:00-5",
                    Alias::new("TIMESTAMP"),
                )
                .into(),
                true.into(),
            ])
            .values_panic([
                "bird".into(),
                Func::cast_as(
                    "2024-06-21 00:00:00-5",
                    Alias::new("TIMESTAMP"),
                )
                .into(),
                true.into(),
            ])
            .values_panic([
                "reptile".into(),
                Func::cast_as(
                    "2024-06-21 00:00:00-5",
                    Alias::new("TIMESTAMP"),
                )
                .into(),
                true.into(),
            ])
            .values_panic([
                "hamster".into(),
                Func::cast_as(
                    "2024-06-21 00:00:00-5",
                    Alias::new("TIMESTAMP"),
                )
                .into(),
                true.into(),
            ])
            .values_panic([
                "other".into(),
                Func::cast_as(
                    "2024-06-21 00:00:00-5",
                    Alias::new("TIMESTAMP"),
                )
                .into(),
                true.into(),
            ])
            .to_owned();

        manager.exec_stmt(insert_initial_pet_types).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PetTypes::Table).cascade().to_owned())
            .await
            .unwrap();

        Ok(())
    }
}
