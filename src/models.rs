use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub contents : String,
    pub completed: bool,
    pub date_created: NaiveDate,
}
