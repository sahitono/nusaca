use crate::models::summary::{DailySummary, RegionSummary};
use sea_orm::{DatabaseConnection, DbBackend, FromQueryResult, Statement};
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
