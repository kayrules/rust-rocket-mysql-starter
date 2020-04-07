use crate::db;
use crate::models::_template_::_Template_;
use crate::utils::error::Error as ApiError;
use crate::utils::response::*;
use crate::*;
use rocket_contrib::json::Json;

#[get("/")]
fn read(conn: db::Connection) -> Result<ApiResponse, ApiError> {
  let result = json!(_Template_::read(&conn)?);
  Ok(success(result))
}

#[get("/<id>")]
fn read_by_id(id: i32, conn: db::Connection) -> Result<ApiResponse, ApiError> {
  let result = json!(_Template_::read_by_id(id, &conn)?);
  Ok(success(result))
}

#[post("/", data = "<_template_>")]
fn create(_template_: Json<_Template_>, conn: db::Connection) -> Result<ApiResponse, ApiError> {
  let insert = _Template_ {
    id: None,
    .._template_.into_inner()
  };
  let result = json!(_Template_::create(insert, &conn)?);
  Ok(success(result))
}

#[put("/<id>", data = "<_template_>")]
fn update(id: i32, _template_: Json<_Template_>, conn: db::Connection) -> Result<ApiResponse, ApiError> {
  let update = _Template_ {
    id: Some(id),
    .._template_.into_inner()
  };
  let result = json!(_Template_::update(id, update, &conn));
  Ok(success(result))
}

#[delete("/<id>")]
fn delete(id: i32, conn: db::Connection) -> Result<ApiResponse, ApiError> {
  let result = json!(_Template_::delete(id, &conn));
  Ok(success(result))
}

// -- routes
pub fn routes() -> Vec<rocket::Route> {
  routes![read, read_by_id, create, update, delete]
}
