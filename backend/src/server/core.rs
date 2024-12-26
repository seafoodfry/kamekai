use axum::http::header::CONTENT_TYPE;
use axum::http::Method;
use axum::{
    extract::{ConnectInfo, MatchedPath},
    http::{header, Request},
    response::Response,
    routing::{get, post},
    Router,
};
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use super::handlers::{handle_health, handle_translate};

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        println!("Received Ctrl+C signal");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
        println!("Received SIGTERM signal");
    };

    #[cfg(unix)]
    let quit = async {
        signal::unix::signal(signal::unix::SignalKind::quit())
            .expect("failed to install signal handler")
            .recv()
            .await;
        println!("Received SIGQUIT signal");
    };

    #[cfg(unix)]
    let interrupt = async {
        signal::unix::signal(signal::unix::SignalKind::interrupt())
            .expect("failed to install signal handler")
            .recv()
            .await;
        println!("Received SIGINT signal");
    };

    #[cfg(unix)]
    let hangup = async {
        signal::unix::signal(signal::unix::SignalKind::hangup())
            .expect("failed to install signal handler")
            .recv()
            .await;
        println!("Received SIGHUP signal");
    };

    #[cfg(unix)]
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
        _ = quit => {},
        _ = interrupt => {},
        _ = hangup => {},
    };

    #[cfg(not(unix))]
    ctrl_c.await;

    println!("Shutting down gracefully...");
}

pub async fn run_server(host: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events.
                format!(
                    "{}=debug,tower_http=info,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with our routes.
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST, Method::OPTIONS])
        .allow_headers([CONTENT_TYPE])
        .max_age(Duration::from_secs(3600));

    let app = Router::new()
        .route("/translate", post(handle_translate))
        .route("/health", get(handle_health))
        .layer(cors) // Need to respond to preflight requests before other middleware interferes/changes headers.
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    let client_ip = request
                        .extensions()
                        .get::<ConnectInfo<SocketAddr>>()
                        .map(|connect_info| connect_info.0.to_string());

                    let headers: Vec<(String, String)> = request
                        .headers()
                        .iter()
                        .map(|(key, value)| {
                            (
                                key.as_str().to_owned(),
                                value.to_str().unwrap_or("invalid").to_owned(),
                            )
                        })
                        .collect();

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        path = %request.uri().path(),
                        matched_path,
                        client_ip = client_ip.as_deref(),
                        headers = ?headers,
                        response.status = tracing::field::Empty,
                        response.size = tracing::field::Empty,
                        response.content_type = tracing::field::Empty,
                        response.latency = tracing::field::Empty,
                    )
                })
                .on_response(|response: &Response, latency: Duration, span: &Span| {
                    let size = response
                        .headers()
                        .get(header::CONTENT_LENGTH)
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse::<usize>().ok())
                        .unwrap_or(0);

                    let content_type = response
                        .headers()
                        .get(header::CONTENT_TYPE)
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("unknown");

                    span.record(
                        "response.status",
                        tracing::field::display(response.status()),
                    );
                    span.record("response.size", size);
                    span.record("response.content_type", content_type);
                    span.record("response.latency_ms", latency.as_millis());

                    tracing::info!(
                        parent: span,
                        "finished processing request"
                    );
                }),
        );

    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(addr).await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}
