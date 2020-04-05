use crate::db;
use crate::models::_template_::_Template_;
use crate::utils::response::*;
use crate::*;
use rocket_contrib::json::Json;

#[get("/")]
fn read(conn: db::Connection) -> ApiResponse {
  api_response!(_Template_::read(&conn))
}

#[get("/<id>")]
fn read_by_id(id: i32, conn: db::Connection) -> ApiResponse {
  api_response!(_Template_::read_by_id(id, &conn))
}

#[post("/", data = "<_template_>")]
fn create(_template_: Json<_Template_>, conn: db::Connection) -> ApiResponse {
  let insert = _Template_ {
    id: None,
    .._template_.into_inner()
  };
  api_response!(_Template_::create(insert, &conn))
}

#[put("/<id>", data = "<_template_>")]
fn update(id: i32, _template_: Json<_Template_>, conn: db::Connection) -> ApiResponse {
  let update = _Template_ {
    id: Some(id),
    .._template_.into_inner()
  };
  api_response!(_Template_::update(id, update, &conn))
}

#[delete("/<id>")]
fn delete(id: i32, conn: db::Connection) -> ApiResponse {
  api_response!(_Template_::delete(id, &conn))
}

// -- routes
pub fn routes() -> Vec<rocket::Route> {
  routes![read, read_by_id, create, update, delete]
}
