use std::net::TcpListener;
use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    )
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(address).expect("Failed to bind random port.");
    run(listener, connection_pool)?.await
}
