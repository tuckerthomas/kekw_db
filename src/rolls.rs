use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::models::period::Period;
use crate::models::roll::{NewRoll, Roll};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;

pub fn get_rolls(pool: &Pool<ConnectionManager<PgConnection>>) -> Vec<Roll> {
    use crate::schema::rolls::dsl::*;

    let results = rolls
        .order(id.desc())
        .limit(5)
        .load::<Roll>(&pool.get().unwrap())
        .expect("Error loading submissions");

    return results;
}

pub fn get_roll_by_period(
    pool: &Pool<ConnectionManager<PgConnection>>,
    search_period: &Period,
) -> Result<Roll> {
    use crate::schema::rolls::dsl::*;

    match Roll::belonging_to(search_period).first::<Roll>(&pool.get()?) {
        Ok(roll) => Ok(roll),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn create_roll(
    pool: &Pool<ConnectionManager<PgConnection>>,
    new_period: &Period,
    new_selection_1: i32,
    new_selection_2: i32,
) -> Result<Roll> {
    use crate::schema::rolls;

    let new_period_id = new_period.id;

    let new_roll = NewRoll {
        period_id: new_period_id,
        selection_1: new_selection_1,
        selection_2: new_selection_2,
    };

    match diesel::insert_into(rolls::table)
        .values(&new_roll)
        .get_result(&pool.get()?)
    {
        Ok(new_roll) => Ok(new_roll),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn delete_roll(pool: &Pool<ConnectionManager<PgConnection>>, del_id: i32) -> Result<usize> {
    use crate::schema::rolls;
    use crate::schema::rolls::dsl::*;

    match diesel::delete(rolls::table.filter(id.eq(del_id))).execute(&pool.get()?) {
        Ok(num_values) => Ok(num_values),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn set_selection_emotes(
    pool: &Pool<ConnectionManager<PgConnection>>,
    mut roll: Roll,
    selection_1_emote: String,
    selection_2_emote: String
) -> Result<Roll> {
    use crate::schema::rolls;

    roll.selection_1_emote = Some(selection_1_emote);
    roll.selection_2_emote = Some(selection_2_emote);

    match diesel::update(rolls::table)
        .set(&roll)
        .get_result(&pool.get()?)
    {
        Ok(updated_roll) => Ok(updated_roll),
        Err(e) => Err(Box::new(e)),
    }
}
