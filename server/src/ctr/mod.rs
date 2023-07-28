mod qr;
use hyper::{Body, Request, Response, StatusCode};
use std::convert::Infallible;

pub async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match req.uri().path() {
        "/qr" => qr::qr_code(req).await,
        _ => Ok(resp(StatusCode::NOT_FOUND)),
    }
}

fn resp(status: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::empty())
        .unwrap()
}
