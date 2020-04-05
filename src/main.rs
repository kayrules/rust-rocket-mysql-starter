#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;

mod controllers;
mod db;
mod models;
mod schema;
mod utils;

// use controllers::user::*;
use rocket_contrib::json::Json;

// use models::user::User;
// use rocket::http::{ContentType, Status};
// use rocket::request::Request;
// use rocket::response::{self, Responder, Response};
// use rocket_contrib::json::{Json, JsonValue};
// use utils::response::*;

// #[post("/", data = "<user>")]
// fn create(user: Json<User>, connection: db::Connection) -> ApiResponse {
//   let insert = User {
//     id: None,
//     ..user.into_inner()
//   };
//   api_response!(User::create(insert, &connection))
// }

// #[get("/")]
// fn read(connection: db::Connection) -> ApiResponse {
//   api_response!(User::read(&connection))
// }

// #[put("/<id>", data = "<user>")]
// fn update(id: i32, user: Json<User>, connection: db::Connection) -> ApiResponse {
//   let update = User {
//     id: Some(id),
//     ..user.into_inner()
//   };
//   api_response!(User::update(id, update, &connection))
// }

// #[delete("/<id>")]
// fn delete(id: i32, connection: db::Connection) -> ApiResponse {
//   api_response!(User::delete(id, &connection))
// }

fn main() {
  rocket::ignite()
    .manage(db::connect())
    .mount("/user", controllers::user::routes())
    .register(utils::catcher::catchers())
    .launch();
}
