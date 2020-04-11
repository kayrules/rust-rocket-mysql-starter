use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::response::Responder;
use rocket_contrib::json::{Json, JsonError, JsonValue};

#[derive(Serialize, Debug)]
pub struct ResponseError {
    pub message: String,
    pub error: String,
    pub code: u16,
}

#[derive(Serialize, Debug)]
pub struct ResponseSuccess {
    pub response: JsonValue,
    pub code: u16,
}

impl ResponseSuccess {
    pub fn ok(response: JsonValue) -> Json<ResponseSuccess> {
        Json(ResponseSuccess {
            code: Status::Ok.code,
            response,
        })
    }
}

#[derive(Responder, Debug)]
pub enum ApiResponse {
    #[response(status = 200, content_type = "json")]
    Ok(Json<ResponseSuccess>),
}

#[derive(Responder, Debug)]
pub enum ApiError {
    #[response(content_type = "json")]
    Bad(Json<ResponseError>),
}

// -- helpers
pub fn success(result: JsonValue) -> ApiResponse {
    ApiResponse::Ok(ResponseSuccess::ok(result))
}

pub fn fail(code: u16, error: String, message: String) -> ApiError {
    ApiError::Bad(Json(ResponseError {
        message,
        error,
        code,
    }))
}

pub fn json_error(e: JsonError) -> ApiError {
    let temp = match e {
        JsonError::Parse(_, error) => format!("{}", error),
        _ => format!("Json syntax error"),
    };

    let status = Status::UnprocessableEntity;
    fail(status.code, status.reason.to_string(), temp)
}

pub fn db_error(e: DieselError) -> ApiError {
    let status = Status::BadRequest;
    fail(status.code, status.reason.to_string(), e.to_string())
}
