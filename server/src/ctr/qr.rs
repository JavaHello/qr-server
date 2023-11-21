use hyper::{header, Body, Request, Response};
use std::convert::Infallible;

use image::{png::PngEncoder, Luma};
use once_cell::sync::Lazy;
#[cfg(feature = "oxipng")]
use oxipng::Options;
use qrcode::QrCode;

static QR_SERVER_KEY: Lazy<String> =
    Lazy::new(|| std::env::var("QR_SERVER_KEY").unwrap_or_else(|_| "qr_server".to_string()));

pub async fn qr_code(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let q = req.uri().query();
    if q.is_none() {
        return Ok(Response::new("null".into()));
    }
    let q = urlencoding::decode(q.unwrap()).unwrap();
    if q.len() > 500 || q.len() < 34 {
        return Ok(Response::new("error param len".into()));
    }
    let b = q.as_bytes();
    let k = QR_SERVER_KEY.as_bytes();
    let k = [&b[32..], k];
    let d = md5::compute(k.concat());
    let d = format!("{:x}", d);
    let c = b.starts_with(d.as_bytes());
    if !c {
        return Ok(Response::new("error param".into()));
    }
    let code = QrCode::new(&b[32..]).unwrap();
    let image = code
        .render::<Luma<u8>>()
        .module_dimensions(600, 600)
        .max_dimensions(600, 600)
        .build();
    let mut buffer = Vec::new();
    PngEncoder::new(&mut buffer)
        .encode(&image, image.width(), image.height(), image::ColorType::L8)
        .unwrap();

    #[cfg(feature = "oxipng")]
    {
        buffer = oxipng::optimize_from_memory(&buffer, &Options::default()).unwrap();
    }
    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "image/png")
        .header(
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"qr.png\"",
        )
        .body(Body::from(buffer))
        .unwrap())
}
