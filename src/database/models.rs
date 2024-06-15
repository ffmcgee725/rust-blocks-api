use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::blocks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BlockQueryModel {
    pub id: i32,
    pub network_id: String,
    pub block_number: i64,
    pub timestamp: i64,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::blocks)]
pub struct BlockInsertModel<'a> {
    pub network_id: &'a str,
    pub block_number: i64,
    pub timestamp: i64,
}
