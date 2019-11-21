#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
use hyper::http::StatusCode;
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server};
use prometheus::{ Encoder,  TextEncoder,Opts};
use std::{net::{ SocketAddr}};
pub use sr_primitives::traits::SaturatedConversion;
pub use prometheus::{Histogram, IntCounter, IntGauge, Result};

pub mod metrics;


/// Initializes the metrics context, and starts an HTTP server
/// to serve metrics.
pub fn init_prometheus(prometheus_addr: SocketAddr) {
    let addr = prometheus_addr;
    let server = Server::bind(&addr)
        .serve(|| {
            // This is the `Service` that will handle the connection.
            // `service_fn_ok` is a helper to convert a function that
            // returns a Response into a `Service`.
            service_fn_ok(move |req: Request<Body>| {
                
                if req.uri().path() == "/metrics" {
                    let metric_families = prometheus::gather();
                    let mut buffer = vec![];
                    let encoder = TextEncoder::new();
                    encoder.encode(&metric_families, &mut buffer).unwrap();
                
                    Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", encoder.format_type())
                        .body(Body::from(buffer))
                        .expect("Sends OK(200) response with one or more data metrics")
                } else {
                    Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::from("Not found."))
                        .expect("Sends NOT_FOUND(404) message with no data metric")
                }
            })
        })
        .map_err(|e| error!("server error: {}", e));

    info!("Exporting metrics at http://{}/metrics", addr);

    let mut rt = tokio::runtime::Builder::new()
        .core_threads(1) // one thread is sufficient
        .build()
        .expect("Builds one thread of tokio runtime exporter for prometheus");

    std::thread::spawn(move || {
        rt.spawn(server);
        rt.shutdown_on_idle().wait().unwrap();
    });
}
