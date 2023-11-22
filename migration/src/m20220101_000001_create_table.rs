use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Region::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Region::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Region::Code).string().not_null())
                    .col(ColumnDef::new(Region::NameEn).string().not_null())
                    .col(ColumnDef::new(Region::NameId).string().not_null())
                    .col(ColumnDef::new(Region::ParentId).integer().null())
                    .col(ColumnDef::new(Region::Longitude).double().not_null())
                    .col(ColumnDef::new(Region::Latitude).double().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Region::Table, Region::ParentId)
                            .to(Region::Table, Region::Id),
                    )
                    .to_owned(),
            )
            .await
            .expect("Failed create city");

        manager
            .create_table(
                Table::create()
                    .table(WeatherParameter::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WeatherParameter::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(WeatherParameter::Description)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
            .expect("Failed create weather parameter");

        manager
            .create_table(
                Table::create()
                    .table(WeatherIssued::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WeatherIssued::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(WeatherIssued::Timestamp)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(WeatherIssued::ProductionCenter)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(WeatherIssued::Source).string().not_null())
                    .col(
                        ColumnDef::new(WeatherIssued::CreatedAt)
                            .timestamp()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
            .expect("Failed to create weather issued");

        manager
            .create_table(
                Table::create()
                    .table(WeatherPrediction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WeatherPrediction::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(WeatherPrediction::RegionId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WeatherPrediction::ParameterId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(WeatherPrediction::Unit).string().not_null())
                    .col(ColumnDef::new(WeatherPrediction::Value).string().not_null())
                    .col(
                        ColumnDef::new(WeatherPrediction::Timestamp)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WeatherPrediction::IssuedId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WeatherPrediction::CreatedAt)
                            .timestamp()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WeatherPrediction::Table, WeatherPrediction::RegionId)
                            .to(Region::Table, Region::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WeatherPrediction::Table, WeatherPrediction::ParameterId)
                            .to(WeatherParameter::Table, WeatherParameter::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WeatherPrediction::Table, WeatherPrediction::IssuedId)
                            .to(WeatherIssued::Table, WeatherPrediction::Id),
                    )
                    .to_owned(),
            )
            .await
            .expect("Failed create weather prediction");

        let insert_wt = Query::insert()
            .into_table(WeatherParameter::Table)
            .columns([WeatherParameter::Id, WeatherParameter::Description])
            .values_panic(["hu".into(), "Humidity".into()])
            .values_panic(["humax".into(), "Max humidity".into()])
            .values_panic(["humin".into(), "Min humidity".into()])
            .values_panic(["t".into(), "Temperature".into()])
            .values_panic(["tmax".into(), "Max temperature".into()])
            .values_panic(["tmin".into(), "Min temperature".into()])
            .values_panic(["weather".into(), "Weather Icon".into()])
            .values_panic(["wd".into(), "Wind Direction".into()])
            .values_panic(["ws".into(), "Wind Speed".into()])
            .to_owned();

        manager
            .exec_stmt(insert_wt)
            .await
            .expect("Failed to seed weather parameter");

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(WeatherPrediction::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
            .expect("Failed drop prediction");

        manager
            .drop_table(
                Table::drop()
                    .table(WeatherIssued::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
            .expect("Failed to drop weather issued");

        manager
            .drop_table(
                Table::drop()
                    .table(WeatherParameter::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
            .expect("Failed drop parameter");
        manager
            .drop_table(Table::drop().table(Region::Table).if_exists().to_owned())
            .await
            .expect("Failed drop city");
        Ok(())
    }
}

use sea_query::Iden;

#[derive(Iden)]
pub enum Region {
    Table,
    Id,
    Code,
    NameEn,
    NameId,
    ParentId,
    Longitude,
    Latitude,
}

#[derive(Iden)]
pub enum WeatherParameter {
    Table,
    Id,
    Description,
}

#[derive(Iden)]
pub enum WeatherIssued {
    Table,
    Id,
    Timestamp,
    CreatedAt,
    Source,
    ProductionCenter,
}

#[derive(Iden)]
pub enum WeatherPrediction {
    Table,
    Id,
    RegionId,
    ParameterId,
    Unit,
    Value,
    Timestamp,
    IssuedId,
    CreatedAt,
}
