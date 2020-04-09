use diesel::result::Error as DieselError;
use rocket::response::Responder;
use rocket_contrib::json::{Json, JsonError, JsonValue};

#[derive(Serialize, Debug)]
pub struct ResponseError {
    pub error: String,
    pub code: u16,
}

#[derive(Serialize, Debug)]
pub struct ResponseSuccess {
    pub response: JsonValue,
    pub code: usize,
}

impl ResponseSuccess {
    pub fn ok(response: JsonValue) -> Json<ResponseSuccess> {
        Json(ResponseSuccess {
            code: 200,
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

pub fn fail(code: u16, message: String) -> ApiError {
    ApiError::Bad(Json(ResponseError {
        error: message,
        code,
    }))
}

pub fn json_error(e: JsonError) -> ApiError {
    let temp = match e {
        JsonError::Parse(_, error) => format!("{}", error),
        _ => format!("Json syntax error"),
    };
    fail(422, temp)
}

pub fn db_error(e: DieselError) -> ApiError {
    fail(500, e.to_string())
}
