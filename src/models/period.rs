use crate::schema::periods;

#[derive(Identifiable, Queryable, AsChangeset, Clone)]
#[changeset_options(treat_none_as_null="true")]
pub struct Period {
    pub id: i32,
    pub start_day: i64,
    pub end_day: Option<i64>,
    pub vote_message: Option<String>
}

#[derive(Insertable)]
#[table_name="periods"]
pub struct NewPeriod {
    pub start_day: i64
}