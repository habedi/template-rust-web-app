use anyhow::Context;
use std::sync::Arc;
use tokio::signal;

use crate::application::use_cases::items::ItemsUseCase;
use crate::config::Config;

mod pg;
mod web;

pub async fn run(config: Config) -> anyhow::Result<()> {
    let pg_pool = config
        .get_pg_pool()
        .context("failed to build the database pool")?;

    sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await
        .context("failed to run database migrations")?;

    let item_service = Box::new(pg::items::PgItemService::new(pg_pool));
    let items = ItemsUseCase::new(item_service);

    web::run(config, web::State::new(Arc::new(items)), shutdown_signal()).await
}

async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(err) = signal::ctrl_c().await {
            tracing::error!("Failed to install Ctrl+C handler: {err}");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match signal::unix::signal(signal::unix::SignalKind::terminate()) {
            Ok(mut sig) => {
                sig.recv().await;
            }
            Err(err) => tracing::error!("Failed to install SIGTERM handler: {err}"),
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutting down...");
}
