use rocket::response::Responder;
use rocket_contrib::json::{Json, JsonValue};

// -- structs & impl
#[derive(Serialize, Debug)]
pub struct Error {
    pub error: String,
    pub code: u16,
}

#[derive(Serialize, Debug)]
pub struct Success {
    pub response: JsonValue,
    pub code: usize,
}

impl Success {
    pub fn ok(response: JsonValue) -> Json<Success> {
        Json(Success {
            code: 200,
            response,
        })
    }
}

#[derive(Responder, Debug)]
pub enum ApiResponse {
    #[response(content_type = "json")]
    Bad(Json<Error>),

    #[response(status = 200, content_type = "json")]
    Ok(Json<Success>),
}

pub fn success(result: JsonValue) -> ApiResponse {
    ApiResponse::Ok(Success::ok(result))
}
