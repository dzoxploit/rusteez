use hyper::{Body, Request, Response};
use hyper::service::service_fn;
use hyper::server::conn::Http;
use prometheus::{Encoder, TextEncoder};
use tokio::net::TcpListener;

async fn handle(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();

    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    Ok(Response::new(Body::from(buffer)))
}

pub fn serve(port: u16) {
    tokio::spawn(async move {
        let listener = TcpListener::bind(("0.0.0.0", port))
            .await
            .expect("failed to bind metrics port");

        loop {
            let (stream, _) = listener.accept().await.unwrap();

            tokio::spawn(async move {
                if let Err(e) = Http::new()
                    .serve_connection(stream, service_fn(handle))
                    .await
                {
                    eprintln!("metrics connection error: {:?}", e);
                }
            });
        }
    });
}
