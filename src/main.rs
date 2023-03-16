
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;
use std::error::Error;

mod models;
mod schema;

use models::{Todo, NewTodo};
use schema::todo;

fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
      .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/todo")]
async fn index() -> Result<HttpResponse, Box<dyn Error>> {
  let connection = &mut establish_connection();
    let todos = todo::table.load::<Todo>(connection)?;
    Ok(HttpResponse::Ok().json(todos))
}

#[post("/todo")]
async fn create(todo: web::Json<NewTodo>) -> Result<HttpResponse, Box<dyn Error>> {
    let connection = &mut establish_connection();
    let todo = todo.into_inner();
    let created_todo = Todo::add_todo(todo.title, todo.contents, todo.completed, todo.date_created, connection)?;
    Ok(HttpResponse::Ok().json(created_todo))
}

#[get("/todo/{id}")]
async fn show(id: web::Path<i32>) -> Result<HttpResponse, Box<dyn Error>> {
    let connection = &mut establish_connection();
    let todo = NewTodo::find(id connection)?;
    Ok(HttpResponse::Ok().json(todo))
}


#[put("/todo/{id}")]
async fn update(
    id: web::Path<i32>,
    todo: web::Json<NewTodo>,
) -> Result<HttpResponse, Box<dyn Error>> {
  let connection = &mut establish_connection();
    let todo = todo.into_inner();
    let updated_todo = Todo::update(id.into_inner(), Some(todo.title), Some(todo.contents), Some(todo.completed), Some(todo.date_created), connection)?;
    Ok(HttpResponse::Ok().json(updated_todo))
}



#[delete("/todo/{id}")]
async fn destroy(id: web::Path<i32>) -> Result<HttpResponse, Box<dyn Error>> {
  let connection = &mut establish_connection();
    let num_deleted = Todo::delete(id.into_inner(), &connection)?;
    Ok(HttpResponse::Ok().json(num_deleted))
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(create)
            .service(show)
            .service(update)
            .service(destroy)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

