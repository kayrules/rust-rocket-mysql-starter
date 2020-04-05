use super::response::*;
use crate::*;

#[catch(422)]
fn unprocessable_entity() -> ApiResponse {
  bad!(422)
}

#[catch(404)]
fn not_found() -> ApiResponse {
  bad!(404)
}

#[catch(500)]
fn internal_server_error() -> ApiResponse {
  bad!(500)
}

// -- catchers
pub fn catchers() -> Vec<rocket::Catcher> {
  catchers![unprocessable_entity, not_found, internal_server_error]
}
