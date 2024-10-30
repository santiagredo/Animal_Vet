use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum UserRoleEvents {
    Table,
    RoleEventId,
    Details,
    CreationDate,
    UserRoleId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRoleEvents::Table)
                    .col(
                        ColumnDef::new(UserRoleEvents::RoleEventId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserRoleEvents::Details).text())
                    .col(ColumnDef::new(UserRoleEvents::CreationDate).timestamp())
                    .col(ColumnDef::new(UserRoleEvents::UserRoleId).integer())
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
                    .table(UserRoleEvents::Table)
                    .cascade()
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }
}
