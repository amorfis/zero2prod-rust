use std::net::TcpListener;

use sqlx::PgPool;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);

    let connection_pool = PgPool::connect_lazy_with(configuration.database.with_db());

    let listener = TcpListener::bind(address).expect("Failed to bind random port.");
    run(listener, connection_pool)?.await
}
