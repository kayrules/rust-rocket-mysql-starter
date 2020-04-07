use super::response::*;
use rocket::http::Status;
use rocket_contrib::json::Json;

fn catcher_response(status: Status) -> ApiResponse {
    ApiResponse::Bad(Json(Error {
        error: status.to_string(),
        code: status.code,
    }))
}

#[catch(400)]
fn bad_request() -> ApiResponse {
    catcher_response(Status::BadRequest)
}

#[catch(422)]
fn unprocessable_entity() -> ApiResponse {
    catcher_response(Status::NotFound)
}

#[catch(404)]
fn not_found() -> ApiResponse {
    catcher_response(Status::NotFound)
}

#[catch(500)]
fn internal_server_error() -> ApiResponse {
    catcher_response(Status::InternalServerError)
}

#[catch(503)]
fn service_unavailable() -> ApiResponse {
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
