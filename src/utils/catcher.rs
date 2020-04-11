use super::response::{fail, ApiError};
use rocket::http::Status;

fn catcher_response(status: Status) -> ApiError {
    fail(status.code, status.reason.to_string(), status.to_string())
}

#[catch(400)]
fn bad_request() -> ApiError {
    catcher_response(Status::BadRequest)
}

#[catch(422)]
fn unprocessable_entity() -> ApiError {
    catcher_response(Status::UnprocessableEntity)
}

#[catch(404)]
fn not_found() -> ApiError {
    catcher_response(Status::NotFound)
}

#[catch(500)]
fn internal_server_error() -> ApiError {
    catcher_response(Status::InternalServerError)
}

#[catch(503)]
fn service_unavailable() -> ApiError {
    catcher_response(Status::ServiceUnavailable)
}

// -- catchers
pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![
        bad_request,
        unprocessable_entity,
        not_found,
        internal_server_error,
        service_unavailable
    ]
}
