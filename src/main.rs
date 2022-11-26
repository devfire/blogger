use std::net::TcpListener;

use blogger::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind to a random port");

    run(listener)?.await
}
