use crate::db;
use crate::models::user::User;
use crate::utils::response::*;
use crate::*;
use rocket_contrib::json::Json;

#[get("/")]
fn read(conn: db::Connection) -> ApiResponse {
  api_response!(User::read(&conn))
}

#[get("/<id>")]
fn read_by_id(id: i32, conn: db::Connection) -> ApiResponse {
  api_response!(User::read_by_id(id, &conn))
}

#[post("/", data = "<user>")]
fn create(user: Json<User>, conn: db::Connection) -> ApiResponse {
  let insert = User {
    id: None,
    ..user.into_inner()
  };
  api_response!(User::create(insert, &conn))
}

#[put("/<id>", data = "<user>")]
fn update(id: i32, user: Json<User>, conn: db::Connection) -> ApiResponse {
  let update = User {
    id: Some(id),
    ..user.into_inner()
  };
  api_response!(User::update(id, update, &conn))
}

#[delete("/<id>")]
fn delete(id: i32, conn: db::Connection) -> ApiResponse {
  api_response!(User::delete(id, &conn))
}

// -- routes
pub fn routes() -> Vec<rocket::Route> {
  routes![read, read_by_id, create, update, delete]
}
