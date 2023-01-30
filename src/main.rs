//use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use diesel::{result::Error};
use crate::{todolist::models::{Todo,NewTodo},};
use serde_json::json;


#[get("/todo")]
async fn find_all() -> Result<HttpResponse, Error> {
    use crate::schema::todo::dsl::todo;
    let todo = Todo::find()?;
    Ok(HttpResponse::Ok().json(todo))
}
#[get("/todo/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let  = Todo:find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[post("/employees")]
async fn create(employee: web::Json<Todo>) -> Result<HttpResponse, Error> {
    let employee =Todo::create(employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[put("/employees/{id}")]
fn update(
    id: web::Path<i32>,
    employee: web::Json<Todo>,
) -> Result<HttpResponse, Error> {
    let employee = Todo::update(id.into_inner(), employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[delete("/employees/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let deleted_employee = Todo::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_employee })))
}







#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}