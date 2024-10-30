use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum UserRoles {
    Table,
    UserRoleId,
    Name,
    IsEnabled,
    IsBackofficeEnabled,
    CreationDate,
    LatestUpdateDate,
    HierarchyLevel,
    UserPermissions,
    PetPermissions,
    PetTypePermissions,
    ServicePermissions,
    WorkDayPermissions,
    AppointmentPermissions,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRoles::Table)
                    .col(
                        ColumnDef::new(UserRoles::UserRoleId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserRoles::Name).text())
                    .col(ColumnDef::new(UserRoles::IsEnabled).boolean())
                    .col(ColumnDef::new(UserRoles::IsBackofficeEnabled).boolean())
                    .col(
                        ColumnDef::new(UserRoles::CreationDate)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserRoles::LatestUpdateDate).timestamp())
                    .col(ColumnDef::new(UserRoles::HierarchyLevel).integer())
                    .col(ColumnDef::new(UserRoles::UserPermissions).integer())
                    .col(ColumnDef::new(UserRoles::PetPermissions).integer())
                    .col(ColumnDef::new(UserRoles::PetTypePermissions).integer())
                    .col(ColumnDef::new(UserRoles::ServicePermissions).integer())
                    .col(ColumnDef::new(UserRoles::WorkDayPermissions).integer())
                    .col(ColumnDef::new(UserRoles::AppointmentPermissions).integer())
                    .to_owned(),
            )
            .await
            .unwrap();

        const CREATE: u32 = 1 << 0;
        const READ: u32 = 1 << 1;
        const UPDATE: u32 = 1 << 2;
        const DELETE: u32 = 1 << 3;

        let crud = CREATE | READ | UPDATE | DELETE;
        let cru = CREATE | READ | UPDATE;
        let cr = CREATE | READ;

        let insert_roles = Query::insert()
            .into_table(UserRoles::Table)
            .columns([
                UserRoles::Name,
                UserRoles::IsEnabled,
                UserRoles::IsBackofficeEnabled,
                UserRoles::CreationDate,
                UserRoles::HierarchyLevel,
                UserRoles::UserPermissions,
                UserRoles::PetPermissions,
                UserRoles::PetTypePermissions,
                UserRoles::ServicePermissions,
                UserRoles::WorkDayPermissions,
                UserRoles::AppointmentPermissions,
            ])
            .values_panic([
                "administrator".into(),
                true.into(),
                true.into(),
                Func::cast_as(
                    "2024-06-17 00:00:00-5",
                    Alias::new("TIMESTAMP"),
                )
                .into(),
                1.into(),
                crud.into(),
                crud.into(),
                crud.into(),
                crud.into(),
                crud.into(),
                crud.into(),
            ])
            .values_panic([
                "staff".into(),
                true.into(),
                true.into(),
                Func::cast_as(
                    "2024-06-17 00:00:00-5",
                    Alias::new("TIMESTAMP"),
                )
                .into(),
                2.into(),
                cru.into(),
                cru.into(),
                cru.into(),
                crud.into(),
                crud.into(),
                cru.into(),
            ])
            .values_panic([
                "user".into(),
                true.into(),
                false.into(),
                Func::cast_as(
                    "2024-06-17 00:00:00-5",
                    Alias::new("TIMESTAMP"),
                )
                .into(),
                3.into(),
                cr.into(),
                cr.into(),
                READ.into(),
                READ.into(),
                READ.into(),
                cru.into(),
            ])
            .to_owned();

        manager.exec_stmt(insert_roles).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRoles::Table).cascade().to_owned())
            .await
            .unwrap();

        Ok(())
    }
}
