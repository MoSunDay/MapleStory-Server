// MapleStory-Server Rust migration
// Entry point - ported from net/server/Server.java

use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("MapleStory-Server Rust (Phase 0 - workspace boot)");
    info!("TODO: Load config, init DB, start login/channel servers");

    Ok(())
}
