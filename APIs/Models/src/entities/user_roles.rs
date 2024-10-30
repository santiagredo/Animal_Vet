//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "user_roles"
    }
}

#[derive(
    Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize, Default,
)]
pub struct Model {
    pub user_role_id: i32,
    pub name: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_backoffice_enabled: Option<bool>,
    pub creation_date: DateTime,
    pub latest_update_date: Option<DateTime>,
    pub hierarchy_level: Option<i32>,
    pub user_permissions: Option<i32>,
    pub pet_permissions: Option<i32>,
    pub pet_type_permissions: Option<i32>,
    pub service_permissions: Option<i32>,
    pub work_day_permissions: Option<i32>,
    pub appointment_permissions: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
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

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    UserRoleId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Users,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::UserRoleId => ColumnType::Integer.def(),
            Self::Name => ColumnType::Text.def().null(),
            Self::IsEnabled => ColumnType::Boolean.def().null(),
            Self::IsBackofficeEnabled => ColumnType::Boolean.def().null(),
            Self::CreationDate => ColumnType::DateTime.def(),
            Self::LatestUpdateDate => ColumnType::DateTime.def().null(),
            Self::HierarchyLevel => ColumnType::Integer.def().null(),
            Self::UserPermissions => ColumnType::Integer.def().null(),
            Self::PetPermissions => ColumnType::Integer.def().null(),
            Self::PetTypePermissions => ColumnType::Integer.def().null(),
            Self::ServicePermissions => ColumnType::Integer.def().null(),
            Self::WorkDayPermissions => ColumnType::Integer.def().null(),
            Self::AppointmentPermissions => ColumnType::Integer.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Users => Entity::has_many(super::users::Entity).into(),
        }
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
