//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "appointments"
    }
}

#[derive(
    Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize, Default,
)]
pub struct Model {
    pub appointment_id: i32,
    pub user_id: Option<i32>,
    pub pet_id: Option<i32>,
    pub date: Option<DateTime>,
    pub service_id: Option<i32>,
    pub is_canceled: Option<bool>,
    pub cancellation_date: Option<DateTime>,
    pub creation_date: Option<DateTime>,
    pub latest_update_date: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    AppointmentId,
    UserId,
    PetId,
    Date,
    ServiceId,
    IsCanceled,
    CancellationDate,
    CreationDate,
    LatestUpdateDate,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    AppointmentId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Pets,
    Services,
    Users,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::AppointmentId => ColumnType::Integer.def(),
            Self::UserId => ColumnType::Integer.def().null(),
            Self::PetId => ColumnType::Integer.def().null(),
            Self::Date => ColumnType::DateTime.def().null(),
            Self::ServiceId => ColumnType::Integer.def().null(),
            Self::IsCanceled => ColumnType::Boolean.def().null(),
            Self::CancellationDate => ColumnType::DateTime.def().null(),
            Self::CreationDate => ColumnType::DateTime.def().null(),
            Self::LatestUpdateDate => ColumnType::DateTime.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Pets => Entity::belongs_to(super::pets::Entity)
                .from(Column::PetId)
                .to(super::pets::Column::PetId)
                .into(),
            Self::Services => Entity::belongs_to(super::services::Entity)
                .from(Column::ServiceId)
                .to(super::services::Column::ServiceId)
                .into(),
            Self::Users => Entity::belongs_to(super::users::Entity)
                .from(Column::UserId)
                .to(super::users::Column::UserId)
                .into(),
        }
    }
}

impl Related<super::pets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pets.def()
    }
}

impl Related<super::services::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Services.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
