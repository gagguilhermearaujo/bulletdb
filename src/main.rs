use bulletdb::run_server;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let listener =
        std::net::TcpListener::bind("127.0.0.1:6644").expect("Failed to bind random port");
    run_server(listener)?.await
}
