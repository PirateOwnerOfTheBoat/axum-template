use crate::startup::run;
use axum_template::{configuration::get_configuration, startup};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let pool = configuration.database.get_pool().unwrap();

    let address = format!("127.0.0.1:{}", configuration.application.port);
    let listener = TcpListener::bind(address).await.unwrap();

    run(listener, pool).unwrap().await
}
