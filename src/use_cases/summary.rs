use crate::models::summary::{DailySummary, RegionSummary};
use entity::weather_prediction;
use sea_orm::{
    DatabaseConnection, DbBackend, DbConn, EntityTrait, FromQueryResult, QuerySelect, Statement,
};
use std::error::Error;

pub async fn get_daily_summary(
    db_conn: &DatabaseConnection,
) -> Result<Vec<DailySummary>, Box<dyn Error>> {
    let daily = DailySummary::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        r#"SELECT DATE(created_at) date, COUNT() as parameter_count
                FROM weather_prediction wp
                GROUP BY DATE(created_at)"#,
        [],
    ))
    .all(db_conn)
    .await
    .unwrap();

    Ok(daily)
}

pub async fn get_region_summary(
    db_conn: &DatabaseConnection,
) -> Result<Vec<RegionSummary>, Box<dyn Error>> {
    let daily = RegionSummary::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        r#"SELECT wp.region_id, r.name_id AS region_name, COUNT() as parameter_count
                FROM weather_prediction wp
                LEFT JOIN main.region r on r.id = wp.region_id
                GROUP BY wp.region_id, r.name_id"#,
        [],
    ))
    .all(db_conn)
    .await
    .unwrap();

    Ok(daily)
}

#[derive(FromQueryResult)]
struct AvailableDate {
    timestamp: String,
}

pub async fn get_available_date(
    db_conn: &DatabaseConnection,
) -> Result<Vec<String>, Box<dyn Error>> {
    let days: Vec<String> = weather_prediction::Entity::find()
        .select_only()
        .column(weather_prediction::Column::Timestamp)
        .group_by(weather_prediction::Column::Timestamp)
        .into_model::<AvailableDate>()
        .all(db_conn)
        .await
        .unwrap()
        .iter()
        .map(|x| x.timestamp.clone())
        .collect();

    Ok(days)
}
