//! Run with
//!
//! ```not_rust
//! cargo run -p example-postgres
//! ```
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::web::App;

#[cfg(test)]
mod test;
mod users;
mod web;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| {
                "axum_login=debug,tower_sessions=debug,sqlx=warn,tower_http=debug,\
                 example_postgres=info"
                    .into()
            },
        )))
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    App::new().await?.serve().await
}