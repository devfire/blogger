use std::net::TcpListener;
use crate::startup::run;

// use blogger::run;
pub mod startup;
pub mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect(r#"Failed to bind."#);

    run(listener)?.await
}
