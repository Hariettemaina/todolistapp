use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::{schema::todo, establish_connection}; //establish_connection};
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


impl Todo {
    pub fn create(
        title: String,
        contents: String,
        completed: bool,
        date_created: NaiveDate,
    ) -> Result<Todo, diesel::result::Error> {
        let new_todo = NewTodo {
            title,
            contents,
            completed,
            date_created,
        };

        let connection = & mut establish_connection();
        let todo = diesel::insert_into(todo::table)
            .values(new_todo)
            .get_result(connection)?;

        Ok(todo)
    }

    pub fn update(
        &self,
        title: Option<String>,
        contents: Option<String>,
        completed: Option<bool>,
        date_created: Option<NaiveDate>,
    ) -> Result<Todo, diesel::result::Error> {
        let connection = & mut establish_connection();

        let todo = diesel::update(todo::table.find(self.id))
            .set((
                todo::title.eq(title.unwrap_or(self.title.clone())),
                todo::contents.eq(contents.unwrap_or(self.contents.clone())),
                todo::completed.eq(completed.unwrap_or(self.completed)),
                todo::date_created.eq(date_created.unwrap_or(self.date_created)),
            ))
            .get_result(connection)?;

        Ok(todo)
    }

    pub fn delete(self) -> Result<usize, diesel::result::Error> {
        let connection = &mut establish_connection();
        let num_deleted = diesel::delete(todo::table.find(self.id)).execute(connection)?;

        Ok(num_deleted)
    }
}

impl NewTodo {
    pub fn find(id: i32) -> Result<Todo, diesel::result::Error> {
        let connection = &mut establish_connection();
        let todo = todo::table.find(id).first::<Todo>(connection)?;

        Ok(todo)
    }
}
