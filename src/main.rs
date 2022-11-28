use std::net::TcpListener;
use blogger::startup::run;
use blogger::config::get_config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    let config = get_config().expect("Failed to read config");
    let address = format!("127.0.0.1:{}",config.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
