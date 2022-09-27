use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::models::period::Period;
use crate::models::submission::Submission;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;

pub fn get_moviesubs(pool: &Pool<ConnectionManager<PgConnection>>, cur_period: &Period) -> Vec<Submission> {
    use crate::schema::submissions::dsl::*;

    let cur_period_id = cur_period.id;

    let results = submissions
        .filter(period_id.eq(cur_period_id))
        .load::<Submission>(&mut pool.get().unwrap())
        .expect("Error loading submissions");

    return results;
}

pub fn get_all_moviesubs(pool: &Pool<ConnectionManager<PgConnection>>) -> Vec<Submission> {
    use crate::schema::submissions::dsl::*;

    let results = submissions
        .load::<Submission>(&mut pool.get().unwrap())
        .expect("Error loading submissions");

    return results;
}

pub fn get_submission_by_id(
    pool: &Pool<ConnectionManager<PgConnection>>,
    search_id: i32,
) -> Result<Submission> {
    use crate::schema::submissions::dsl::*;

    match submissions
        .filter(id.eq(search_id))
        .first::<Submission>(&mut pool.get()?)
    {
        Ok(submission) => Ok(submission),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn get_submission_by_period_and_user(
    pool: &Pool<ConnectionManager<PgConnection>>,
    search_period: &Period,
    user_id: String,
) -> Result<Submission> {
    use crate::schema::submissions::dsl::*;

    match Submission::belonging_to(search_period)
        .filter(dis_user_id.eq(user_id))
        .first::<Submission>(&mut pool.get()?)
    {
        Ok(submission) => Ok(submission),
        Err(e) => Err(Box::new(e)),
    }
}

use crate::models::submission::NewSubmission;
pub fn create_moviesub<'a>(
    conn: &mut PgConnection,
    dis_user_id: &'a str,
    title: &'a str,
    link: &'a str,
    period_id: i32,
) -> usize {
    use crate::schema::submissions;

    let new_post = NewSubmission {
        dis_user_id: dis_user_id,
        title: title,
        link: link,
        period_id: period_id,
    };

    diesel::insert_into(submissions::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new submission")
}

pub fn update_moviesub<'a>(
    pool: &Pool<ConnectionManager<PgConnection>>,
    moviesub_to_update: Submission,
) -> Result<usize> {
    match diesel::update(&moviesub_to_update)
        .set(&moviesub_to_update)
        .execute(&mut pool.get()?)
    {
        Ok(num_values) => Ok(num_values),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn delete_moviesub<'a>(
    pool: &Pool<ConnectionManager<PgConnection>>,
    del_submission: &Submission,
) -> usize {
    use crate::schema::submissions;
    use crate::schema::submissions::dsl::*;

    diesel::delete(del_submission)
        .execute(&mut pool.get().unwrap())
        .expect("Error deleting submission")
}

pub fn check_prev_sub<'a>(
    conn: &mut PgConnection,
    cur_period_id: i32,
    check_dis_user_id: &'a str,
) -> Vec<Submission> {
    use crate::schema::submissions::dsl::*;

    let results = submissions
        .filter(
            period_id
                .eq(cur_period_id)
                .and(dis_user_id.eq(check_dis_user_id)),
        )
        .load::<Submission>(conn)
        .expect("Error loading submissions");

    return results;
}
