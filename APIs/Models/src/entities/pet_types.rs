//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "pet_types"
    }
}

#[derive(
    Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize, Default,
)]
pub struct Model {
    pub pet_type_id: i32,
    pub name: Option<String>,
    pub creation_date: Option<DateTime>,
    pub latest_update_date: Option<DateTime>,
    pub is_enabled: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    PetTypeId,
    Name,
    CreationDate,
    LatestUpdateDate,
    IsEnabled,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    PetTypeId,
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
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::PetTypeId => ColumnType::Integer.def(),
            Self::Name => ColumnType::Text.def().null(),
            Self::CreationDate => ColumnType::DateTime.def().null(),
            Self::LatestUpdateDate => ColumnType::DateTime.def().null(),
            Self::IsEnabled => ColumnType::Boolean.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Pets => Entity::has_many(super::pets::Entity).into(),
        }
    }
}

impl Related<super::pets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pets.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
