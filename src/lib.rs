use actix_web::{App, HttpServer};
use std;

mod route_get;
mod route_set;
mod tests;

use std::net::TcpListener;

pub fn run_server(listener: TcpListener) -> Result<actix_web::dev::Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(route_set::set).service(route_get::get))
        .listen(listener)?
        .run();
    Ok(server)
}

pub fn spawn_test_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run_server(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
