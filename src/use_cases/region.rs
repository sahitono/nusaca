use entity::region;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryTrait};
use sea_query::Condition;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
pub struct Params {
    keyword: Option<String>,
}

pub async fn get_regions(
    db_conn: &DatabaseConnection,
    keyword: Option<String>,
) -> Result<Vec<region::Model>, Box<dyn Error>> {
    let regions = region::Entity::find()
        .apply_if(Some(keyword), |mut query, v| {
            if v.is_some() {
                let keyword = v.unwrap();
                query.filter(
                    Condition::any()
                        .add(region::Column::NameId.contains(&keyword))
                        .add(region::Column::NameEn.contains(&keyword)),
                )
            } else {
                query
            }
        })
        .all(db_conn)
        .await
        .unwrap();

    Ok(regions)
}

pub async fn get_region_by_code(
    db_conn: &DatabaseConnection,
    code: &str,
) -> Result<Vec<region::Model>, Box<dyn Error>> {
    let regions = region::Entity::find()
        .filter(region::Column::Code.eq(code))
        .all(db_conn)
        .await
        .unwrap();

    Ok(regions)
}
