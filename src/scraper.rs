use bmkg_opendata::model::Area;
use entity::region::ActiveModel;
use entity::{region, weather_issued, weather_prediction};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};

pub async fn scrape_weathers(db_conn: &DbConn) -> Result<(), Box<dyn std::error::Error>> {
    let regions = region::Entity::find()
        .filter(region::Column::ParentId.eq(1))
        .all(db_conn)
        .await?;

    for region in regions {
        let res = scrape_weather(&region.name_en, &region.id, db_conn).await;
        if res.is_err() {
            tracing::info!("Failed to generate: {}", region.name_en);
            continue;
        }
    }

    Ok(())
}

pub async fn scrape_weather(
    parent_region_name: &str,
    parent_region_id: &i32,
    db_conn: &DbConn,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::debug!("fetching: {}", parent_region_name);
    let forecast =
        bmkg_opendata::scraper::scrape_weather(&parent_region_name.replace(' ', "")).await?;

    tracing::debug!("storing to db: {}", parent_region_name);
    let forecast_data = forecast.forecast.first().unwrap().clone();

    let existing_weather = weather_issued::Entity::find()
        .filter(weather_issued::Column::Timestamp.eq(&forecast_data.issue.timestamp))
        .all(db_conn)
        .await?;

    if !existing_weather.is_empty() {
        tracing::debug!("weather already exist, skipping");
        return Ok(());
    };

    let issued = weather_issued::ActiveModel {
        timestamp: Set(forecast_data.issue.timestamp.clone()),
        source: Set(forecast.source),
        production_center: Set(forecast.production_center),
        ..Default::default()
    }
    .save(db_conn)
    .await
    .unwrap();

    let issued_id = issued.id.unwrap();

    for area in &forecast_data.area {
        // let trx = db_conn.begin().await?;

        if area.parameter.is_none() {
            tracing::debug!("area empty: {}", &area.id);
            continue;
        }

        let db_region = region::Entity::find()
            .filter(region::Column::Code.eq(&area.id))
            .one(db_conn)
            .await?;
        let region_id = match db_region {
            Some(reg) => reg.id,
            None => {
                tracing::debug!("creating new region: {}", &area.id);
                let result = create_region(&area, db_conn, parent_region_id).await;
                match result {
                    Ok(reg) => reg.id.unwrap(),
                    Err(err) => {
                        tracing::error!("Failed to create region: {}", &area.id);
                        tracing::debug!("{}", err.to_string());
                        continue;
                    }
                }
            }
        };

        let parameters = area.parameter.as_deref().unwrap();

        for parameter in parameters {
            for time_range in &parameter.time_range {
                for value in &time_range.value {
                    let result = weather_prediction::ActiveModel {
                        value: Set(value.value.clone()),
                        region_id: Set(region_id),
                        timestamp: Set(time_range.datetime.clone()),
                        unit: Set(value.unit.clone()),
                        parameter_id: Set(parameter.id.clone()),
                        issued_id: Set(issued_id),
                        ..Default::default()
                    }
                    .save(db_conn)
                    .await;
                    match result {
                        Ok(_) => {}
                        Err(err) => {
                            tracing::error!("Failed to create parameter: {}", parameter.r#type);
                            tracing::debug!("{}", err.to_string());
                            continue;
                        }
                    }
                }
            }
        }

        // trx.commit().await?;
    }
    Ok(())
}

async fn create_region(
    area: &Area,
    db_conn: &DbConn,
    parent_region_id: &i32,
) -> Result<ActiveModel, DbErr> {
    let mut name_en: String = "".to_string();
    let mut name_id: String = "".to_string();

    for name in area.name.iter() {
        match &*name.lang {
            "en_US" => name_en = name.value.clone(),
            "id_ID" => name_id = name.value.clone(),
            _ => {}
        }
    }

    region::ActiveModel {
        code: Set(area.id.to_string()),
        latitude: Set(area.latitude),
        longitude: Set(area.longitude),
        name_en: Set(name_en),
        name_id: Set(name_id),
        parent_id: Set(Some(*parent_region_id)),
        ..Default::default()
    }
    .save(db_conn)
    .await
}
