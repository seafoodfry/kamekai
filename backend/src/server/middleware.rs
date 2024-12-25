use hyper::Request;

pub async fn log_request<B>(req: Request<B>, addr: std::net::SocketAddr) -> Request<B> {
    println!(
        "Received {} request to {} from {}. Headers {:#?}",
        req.method(),
        req.uri(),
        addr,
        req.headers()
    );
    req
}
