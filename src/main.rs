use std::error::Error;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use crate::models::{Todo,NewTodo};
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;

pub mod todo;
pub mod models;
pub mod schema;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}









#[actix_web::main]


async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}