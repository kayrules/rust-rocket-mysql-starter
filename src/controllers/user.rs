use crate::db;
use crate::models::user::User;
use crate::utils::error::Error as ApiError;
use crate::utils::response::*;
use crate::*;
use rocket_contrib::json::Json;

#[get("/")]
fn read(conn: db::Connection) -> Result<ApiResponse, ApiError> {
    let result = json!(User::read(&conn)?);
    Ok(success(result))
}

#[get("/<id>")]
fn read_by_id(id: i32, conn: db::Connection) -> Result<ApiResponse, ApiError> {
    let result = json!(User::read_by_id(id, &conn)?);
    Ok(success(result))
}

#[post("/", data = "<user>")]
fn create(user: Json<User>, conn: db::Connection) -> Result<ApiResponse, ApiError> {
    let insert = User {
        id: None,
        ..user.into_inner()
    };
    let result = json!(User::create(insert, &conn)?);
    Ok(success(result))
}

#[put("/<id>", data = "<user>")]
fn update(id: i32, user: Json<User>, conn: db::Connection) -> Result<ApiResponse, ApiError> {
    let update = User {
        id: Some(id),
        ..user.into_inner()
    };
    let result = json!(User::update(id, update, &conn));
    Ok(success(result))
}

#[delete("/<id>")]
fn delete(id: i32, conn: db::Connection) -> Result<ApiResponse, ApiError> {
    let result = json!(User::delete(id, &conn));
    Ok(success(result))
}

// -- routes
pub fn routes() -> Vec<rocket::Route> {
    routes![read, read_by_id, create, update, delete]
}
