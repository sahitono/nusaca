use entity::region;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryTrait};
use sea_query::Condition;
use std::error::Error;

pub async fn get_regions(
    db_conn: &DatabaseConnection,
    keyword: Option<String>,
    parent_id: Option<i32>,
) -> Result<Vec<region::Model>, Box<dyn Error>> {
    let regions = region::Entity::find()
        .apply_if(keyword, |query, v| {
            query.filter(
                Condition::any()
                    .add(region::Column::NameId.contains(&v))
                    .add(region::Column::NameEn.contains(&v)),
            )
        })
        .apply_if(parent_id, |query, v| {
            query.filter(region::Column::ParentId.eq(parent_id))
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
