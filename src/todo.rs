// use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
// use diesel::{prelude::*, result::Error};
// use crate::{models::{Todo,NewTodo}, establish_connection};
// use serde_json::json;



// use crate::{}::{Employee, Employees};
// use crate::error_handler::CustomError;
// use actix_web::{delete, get, post, put, web, HttpResponse};
// use serde_json::json;

// #[get("/todo")]
// async fn find_all() -> Result<HttpResponse, Error> {
//     use crate::schema::todo::dsl::todo;
//     let employees = Todo::find()?;
//     Ok(HttpResponse::Ok().json(todo))
// }

// #[get("/todo/{id}")]
// async fn find(id: web::Path<i32>) -> Result<HttpResponse, Error> {
//     let employee = Todo:find(id.into_inner())?;
//     Ok(HttpResponse::Ok().json(employee))
// }

// #[post("/employees")]
// async fn create(employee: web::Json<Todo>) -> Result<HttpResponse, Error> {
//     let employee =Todo::create(employee.into_inner())?;
//     Ok(HttpResponse::Ok().json(employee))
// }

// #[put("/employees/{id}")]
// fn update(
//     id: web::Path<i32>,
//     employee: web::Json<Todo>,
// ) -> Result<HttpResponse, Error> {
//     let employee = Todo::update(id.into_inner(), employee.into_inner())?;
//     Ok(HttpResponse::Ok().json(employee))
// }

// #[delete("/employees/{id}")]
// async fn delete(id: web::Path<i32>) -> Result<HttpResponse, Error> {
//     let deleted_employee = Todo::delete(id.into_inner())?;
//     Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_employee })))
// }

// pub fn init_routes(comfig: &mut web::ServiceConfig) {
//     comfig.service(find_all);
//     comfig.service(find);
//     comfig.service(create);
//     comfig.service(update);
//     comfig.service(delete);
// }
