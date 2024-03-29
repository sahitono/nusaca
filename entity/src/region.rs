//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "region")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub code: String,
    pub name_en: String,
    pub name_id: String,
    pub parent_id: Option<i32>,
    #[sea_orm(column_type = "Double")]
    pub longitude: f64,
    #[sea_orm(column_type = "Double")]
    pub latitude: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SelfRef,
    #[sea_orm(has_many = "super::weather_prediction::Entity")]
    WeatherPrediction,
}

impl Related<super::weather_prediction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WeatherPrediction.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
