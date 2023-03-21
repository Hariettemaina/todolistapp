use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

mod models;
mod schema;

use models::{NewTodo, Todo};

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// Endpoint to get all Todos
#[get("/todos")]
async fn get_all_todos() -> impl Responder {
    let connection = &mut establish_connection();
    let todos = schema::todo::table.load::<Todo>(connection).unwrap();

    HttpResponse::Ok().json(todos)
}

// Endpoint to create a new Todo
#[post("/todos")]
async fn create_todo(new_todo: web::Json<NewTodo>) -> impl Responder {
    let todo = Todo::add_todo(
        new_todo.title.clone(),
        new_todo.contents.clone(),
        new_todo.completed.clone(),
        new_todo.date_created.clone(),
    )
    .unwrap();

    HttpResponse::Created().json(todo)
}

// Endpoint to update an existing Todo
#[put("/todos/{id}")]
async fn update_todo(path: web::Path<i32>, new_todo: web::Json<NewTodo>) -> impl Responder {
    let todo = Todo::find(path.into_inner()).unwrap();
    let updated_todo = todo
        .update(
            new_todo.title.clone(),
            new_todo.contents.clone(),
            new_todo.completed.clone(),
            new_todo.date_created.clone(),
        )
        .unwrap();

    HttpResponse::Ok().json(updated_todo)
}

// Endpoint to delete a Todo
#[delete("/todos/{id}")]
async fn delete_todo(path: web::Path<i32>) -> impl Responder {
    let todo = Todo::find(path.into_inner()).unwrap();
    let num_deleted = todo.delete().unwrap();

    HttpResponse::Ok().json(num_deleted)
}

// Run the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_all_todos)
            .service(create_todo)
            .service(update_todo)
            .service(delete_todo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
