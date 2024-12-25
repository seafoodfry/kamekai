use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use super::handlers::{handle_not_found, handle_translate};
use super::middleware::log_request;

// // The executor is essential for HTTP/2's concurrent stream handling.
// // It determines how the server manages multiple streams within a single connection.
// #[derive(Clone)]
// struct TokioExecutor;

// impl<F> hyper::rt::Executor<F> for TokioExecutor
// where
//     F: std::future::Future + Send + 'static,
//     F::Output: Send + 'static,
// {
//     fn execute(&self, fut: F) {
//         tokio::task::spawn(fut);
//     }
// }

async fn handle_request(
    req: Request<hyper::body::Incoming>,
    client_addr: std::net::SocketAddr,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let req = log_request(req, client_addr).await;

    // Since both handlers return Result<Response, Infallible>, we need to convert
    // the Result type to match our function's return type
    match (req.method(), req.uri().path()) {
        (&hyper::Method::POST, "/translate") => {
            // Convert Result<Response, Infallible> to Result<Response, hyper::Error>
            // We can safely map_err here because Infallible can never actually occur
            handle_translate(req)
                .await
                .map_err(|infallible| match infallible {})
        }
        _ => handle_not_found(req)
            .await
            .map_err(|infallible| match infallible {}),
    }
}

// Our streamlined server implementation focused on HTTP/2.
pub async fn run_server(host: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).await?;
    println!("HTTP/2 server running on http://{}", addr);

    loop {
        let (stream, client_addr) = listener.accept().await?;
        println!("New connection from: {}", client_addr);

        let io = TokioIo::new(stream);

        // Configure and spawn HTTP/2 connection handler.
        tokio::task::spawn(async move {
            // For http2: http2::Builder::new(TokioExecutor)
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| handle_request(req, client_addr)))
                .await
            {
                eprintln!("Error serving connection from {}: {:?}", client_addr, err);
            }
        });
    }
}
