#[macro_export]
macro_rules! bad {
  ($e: expr) => {
    match $e {
      404 => ApiResponse::NotFound(Json(Error::not_found())),
      422 => ApiResponse::Unprocessable(Json(Error::unprocessable_entity())),
      500 => ApiResponse::InternalServerError(Json(Error::internal_server_error())),
      _ => ApiResponse::BadRequest(Json(Error::bad_request())),
    }
  };
}

#[macro_export]
macro_rules! ok {
  ($e: expr) => {
    ApiResponse::Success(Json(Success::ok(json!($e))));
  };
}

#[macro_export]
macro_rules! api_response {
  ($e: expr) => {
    #[allow(irrefutable_let_patterns)]
    if let response = $e {
      ok!(response)
    } else {
      bad!(400)
    }
  };
}
