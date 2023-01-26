use chrono::NaiveDate;
 use diesel::RunQueryDsl;
use std::error::Error;
use crate::{models::{NewTodo,Todo}, establish_connection};



pub fn create_todo(
    title: String,
    contents: String,
    completed: bool,
    date_created: NaiveDate,
) ->Result<Todo, Box<dyn Error>> {
    use crate::schema::todo::dsl::todo;
    let connection = &mut establish_connection();

    let new_todo = NewTodo{
        title,
        contents,
        completed,date_created,
    };
    let results = diesel::insert_into(todo)
         .values(new_todo)
         .get_result(connection)
        .expect("Error inserting to the database");

    Ok(results)

    
}
