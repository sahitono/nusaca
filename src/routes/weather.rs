use crate::AppState;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use chrono::format::Fixed::TimezoneOffset;
use chrono::{FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use entity::{region, weather_prediction};
use nusaca::base::response::BaseResponse;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{
    ColumnTrait, DbBackend, EntityTrait, QueryFilter, QuerySelect, QueryTrait, Related,
    RelationTrait,
};
use sea_query::Condition;
use sea_query::JoinType;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct Params {
    #[serde(rename = "regionName")]
    region_name: Option<String>,
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[serde(rename = "regionCode")]
    region_code: Option<String>,
    parameter: Option<String>,
    date: Option<NaiveDate>,
}

pub async fn get_predictions(
    State(app): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let date = params.date.unwrap_or(Local::now().date_naive());
    let timestamp = date.format("%Y%m%d");

    let query = weather_prediction::Entity::find()
        .join(JoinType::Join, weather_prediction::Relation::Region.def())
        .filter(
            Condition::all()
                .add(region::Column::Code.eq(params.region_code))
                .add(weather_prediction::Column::Timestamp.like(format!("{}%", timestamp))),
        )
        .apply_if(params.parameter, |q, v| {
            // match v {
            //     Some(v) => q.filter(weather_prediction::Column::ParameterId.eq(v)),
            //     None => q,
            // }
            q.filter(weather_prediction::Column::ParameterId.eq(v))
        });

    let models = query.all(&app.db_connection).await.unwrap();

    Json(BaseResponse { data: models })
}
