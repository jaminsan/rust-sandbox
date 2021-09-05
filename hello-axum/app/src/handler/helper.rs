use axum::body::Body;
use axum::http::StatusCode;
use hyper::Response;

pub fn internal_server_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
        .unwrap()
}
pub fn bad_request() -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::empty())
        .unwrap()
}