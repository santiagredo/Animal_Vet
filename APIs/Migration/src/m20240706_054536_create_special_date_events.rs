use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum SpecialDateEvents {
    Table,
    SpecialDateEventId,
    SpecialDateId,
    Details,
    CreationDate,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SpecialDateEvents::Table)
                    .col(
                        ColumnDef::new(SpecialDateEvents::SpecialDateEventId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SpecialDateEvents::SpecialDateId).integer())
                    .col(ColumnDef::new(SpecialDateEvents::Details).text())
                    .col(ColumnDef::new(SpecialDateEvents::CreationDate).timestamp())
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
                    .table(SpecialDateEvents::Table)
                    .cascade()
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }
}
