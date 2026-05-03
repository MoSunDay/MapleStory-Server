// MapleStory-Server Rust entry point
// Ported from net/server/Server.java

mod login;

use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    info!("MapleStory-Server Rust v0.1.0 starting...");
    info!("Server version: v83");

    login::run_login_server().await?;

    Ok(())
}
