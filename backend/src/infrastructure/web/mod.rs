use axum::Router;
use std::{future::Future, sync::Arc};
use tower_http::trace;

use crate::{application::use_cases::items::ItemsUseCaseTrait, config::Config};

mod error;
mod middleware;
mod routes;

#[derive(Clone)]
pub struct State {
    items: Arc<dyn ItemsUseCaseTrait>,
}

impl State {
    pub fn new(items: Arc<dyn ItemsUseCaseTrait>) -> Self {
        State { items }
    }
}

pub async fn run(
    config: Config,
    state: State,
    shutdown_signal: impl Future<Output = ()> + Send + 'static,
) -> anyhow::Result<()> {
    let app = Router::new()
        .merge(routes::health::router())
        .nest("/api/v1/items", routes::items::router())
        .with_state(state)
        .layer(config.get_cors_layer())
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO))
                .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(tracing::Level::ERROR)),
        );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{0}", config.port)).await?;

    if let Ok(addr) = listener.local_addr() {
        tracing::info!("Listening on http://{addr}");
    }

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await?;

    Ok(())
}

#[cfg(test)]
pub fn get_mock_state(items: crate::application::use_cases::items::MockItemsUseCase) -> State {
    State {
        items: Arc::new(items),
    }
}
