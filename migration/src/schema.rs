use sea_query::Iden;

#[derive(Iden)]
pub enum Province {
    Table,
    Id,
    Code,
    NameEn,
    NameId,
}

#[derive(Iden)]
pub enum City {
    Table,
    Id,
    Code,
    NameEn,
    NameId,
}
