use blogger::config::get_config;
use blogger::startup::run;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read config");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    let connection = PgConnection::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    run(listener, connection)?.await
}
