use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum PetTypeEvents {
    Table,
    PetTypeEventId,
    PetTypeId,
    Details,
    CreationDate,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PetTypeEvents::Table)
                    .col(
                        ColumnDef::new(PetTypeEvents::PetTypeEventId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PetTypeEvents::PetTypeId).integer())
                    .col(ColumnDef::new(PetTypeEvents::Details).text())
                    .col(ColumnDef::new(PetTypeEvents::CreationDate).timestamp())
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
                    .table(PetTypeEvents::Table)
                    .cascade()
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }
}
