use rocket::response::Responder;
use rocket_contrib::json::{Json, JsonValue};

// -- structs & impl
#[derive(Serialize, Debug)]
pub struct Error {
  pub message: String,
  pub error: String,
  pub code: usize,
}

impl Error {
  pub fn bad_request() -> Error {
    return Error {
      message: String::from("Bad Request"),
      error: String::from("400 Bad Request"),
      code: 400,
    };
  }

  pub fn internal_server_error() -> Error {
    return Error {
      message: String::from("Internal Server Error"),
      error: String::from("500 Internal Server Error"),
      code: 500,
    };
  }

  pub fn unprocessable_entity() -> Error {
    return Error {
      message: String::from("Unprocessable Entity"),
      error: String::from("422 Unprocessable Entity"),
      code: 422,
    };
  }

  pub fn not_found() -> Error {
    return Error {
      message: String::from("Not Found"),
      error: String::from("404 Not Found"),
      code: 404,
    };
  }
}

#[derive(Serialize, Debug)]
pub struct Success {
  pub response: JsonValue,
  pub code: usize,
}

impl Success {
  pub fn ok(response: JsonValue) -> Success {
    return Success {
      code: 200,
      response,
    };
  }
}

#[derive(Responder, Debug)]
pub enum ApiResponse {
  #[response(status = 400, content_type = "json")]
  BadRequest(Json<Error>),

  #[response(status = 422, content_type = "json")]
  Unprocessable(Json<Error>),

  #[response(status = 404, content_type = "json")]
  NotFound(Json<Error>),

  #[response(status = 500, content_type = "json")]
  InternalServerError(Json<Error>),

  #[response(status = 200, content_type = "json")]
  Success(Json<Success>),
}
