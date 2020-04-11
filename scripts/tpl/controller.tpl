use crate::db;
use crate::models::_template_::_Template_;
use crate::utils::response::*;
use crate::*;
use rocket_contrib::json::{Json, JsonError};

#[get("/")]
fn read(conn: db::Connection) -> Result<ApiResponse, ApiError> {
    let result = _Template_::read(&conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

#[get("/<id>")]
fn read_by_id(id: i32, conn: db::Connection) -> Result<ApiResponse, ApiError> {
    let result = _Template_::read_by_id(id, &conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

#[post("/", data = "<_template_>")]
fn create(
    _template_: Result<Json<_Template_>, JsonError>,
    conn: db::Connection,
) -> Result<ApiResponse, ApiError> {
    match _template_ {
        Ok(u) => {
            let insert = _Template_ {
                id: None,
                ..u.into_inner()
            };
            let result = _Template_::create(insert, &conn);
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

#[put("/<id>", data = "<_template_>")]
fn update(
    id: i32,
    _template_: Result<Json<_Template_>, JsonError>,
    conn: db::Connection,
) -> Result<ApiResponse, ApiError> {
    match _template_ {
        Ok(u) => {
            let update = _Template_ {
                id: Some(id),
                ..u.into_inner()
            };
            let result = _Template_::update(id, update, &conn);
	    match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }

        }
        Err(e) => Err(json_error(e)),
    }
}

#[delete("/<id>")]
fn delete(id: i32, conn: db::Connection) -> ApiResponse {
    let result = _Template_::delete(id, &conn);
    success(json!(result))
}

// -- routes
pub fn routes() -> Vec<rocket::Route> {
    routes![read, read_by_id, create, update, delete]
}
