mod ctr;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("QR_SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("error port");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(ctr::router)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("server start port: {}", port);
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
    println!("server stop port: {}", port);
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
