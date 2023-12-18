use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct RegionParameterSummary {
    region_id: String,
    region_name: String,
    parameter_id: String,
    parameter_count: i32,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct RegionSummary {
    region_id: i32,
    #[serde(rename(deserialize = "name_id"))]
    region_name: String,
    parameter_count: i64,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct ParameterSummary {
    parameter_id: String,
    parameter_count: i32,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct DailySummary {
    date: String,
    parameter_count: i64,
}
