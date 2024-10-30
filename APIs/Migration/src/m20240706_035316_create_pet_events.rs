use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum PetEvents {
    Table,
    PetEventId,
    PetId,
    Details,
    CreationDate,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PetEvents::Table)
                    .col(
                        ColumnDef::new(PetEvents::PetEventId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PetEvents::PetId).integer())
                    .col(ColumnDef::new(PetEvents::Details).text())
                    .col(ColumnDef::new(PetEvents::CreationDate).timestamp())
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PetEvents::Table).cascade().to_owned())
            .await
            .unwrap();

        Ok(())
    }
}
