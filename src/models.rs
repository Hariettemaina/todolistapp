use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::todo;
#[derive(Queryable,Serialize,Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub contents : String,
    pub completed: bool,
    pub date_created: NaiveDate,
}


#[derive(Insertable)]
#[table_name="todo"]
pub struct NewTodo {
    pub title: String,
    pub contents : String,
    pub completed: bool,
    pub date_created: NaiveDate,
}