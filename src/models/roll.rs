use crate::schema::rolls;
use crate::models::period::Period;

#[derive(Identifiable, Queryable, Associations, PartialEq, AsChangeset, Debug)]
#[belongs_to(Period)]
pub struct Roll {
    pub id: i32,
    pub selection_1: i32,
    pub selection_2: i32,
    pub period_id: i32,
    pub selection_1_emote: Option<String>,
    pub selection_2_emote: Option<String>
}

#[derive(Insertable, Associations, PartialEq, Debug)]
#[belongs_to(Period)]
#[table_name="rolls"]
pub struct NewRoll {
    pub selection_1: i32,
    pub selection_2: i32,
    pub period_id: i32
}