use crate::db;
use crate::models::user::User;
use crate::utils::response::*;
use crate::*;
use rocket_contrib::json::{Json, JsonError};

#[get("/")]
fn read(conn: db::Connection) -> Result<ApiResponse, ApiError> {
    let result = User::read(&conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

#[get("/<id>")]
fn read_by_id(id: i32, conn: db::Connection) -> Result<ApiResponse, ApiError> {
    let result = User::read_by_id(id, &conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

#[post("/", data = "<user>")]
fn create(
    user: Result<Json<User>, JsonError>,
    conn: db::Connection,
) -> Result<ApiResponse, ApiError> {
    match user {
        Ok(u) => {
            let insert = User {
                id: None,
                ..u.into_inner()
            };
            let result = User::create(insert, &conn);
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

#[put("/<id>", data = "<user>")]
fn update(
    id: i32,
    user: Result<Json<User>, JsonError>,
    conn: db::Connection,
) -> Result<ApiResponse, ApiError> {
    match user {
        Ok(u) => {
            let update = User {
                id: Some(id),
                ..u.into_inner()
            };
            let result = json!(User::update(id, update, &conn));
            Ok(success(result))
        }
        Err(e) => Err(json_error(e)),
    }
}

#[delete("/<id>")]
fn delete(id: i32, conn: db::Connection) -> ApiResponse {
    let result = User::delete(id, &conn);
    success(json!(result))
}

// -- routes
pub fn routes() -> Vec<rocket::Route> {
    routes![read, read_by_id, create, update, delete]
}
