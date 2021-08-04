use tracing_subscriber::{prelude::*, EnvFilter};

pub fn initialize_logs() {
    let filter_layer = EnvFilter::from_default_env()
        .add_directive("axum_playground=trace".parse().unwrap())
        .add_directive("debug".parse().unwrap());

    let fmt_layer = tracing_subscriber::fmt::layer().without_time();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
