#substrate Monitor tool prometheus

## 소개

프로메테우스는, 다른 체인의 생태계에서 매우 유용하게 사용되고있다. 코어에 포함 되게 되면 좀 더 세밀한 metrics 구현이 가능하다. 여러가지 기능들이
있는데, 주로 노드헬스, 체인헬스에 대한 타임라인관리, 실시간알람, 그리고 그라파나를 이용한 시각화를 통해 노드를 철저하게 관리 할 수있다.

## Metrics

substrate can report and serve the Prometheus metrics, which in their turn can be consumed by Prometheus collector(s).

This functionality is disabled by default.

To enable the Prometheus metrics, set in your cli command (--prometheus-addr,--prometheus-port ). 
Metrics will be served under /metrics on 33333 port by default.

## Substrate Dev hack
### Prometheus starter

client/prometheus/src/lib.rs
```rust
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
                        .expect("Error constructing response")
                } else {
                    Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::from("Not found."))
                        .expect("Error constructing response")
                }
            })
        })
        .map_err(|e| error!("server error: {}", e));

    info!("Exporting metrics at http://{}/metrics", addr);

    let mut rt = tokio::runtime::Builder::new()
        .core_threads(1) // one thread is sufficient
        .build()
        .expect("Unable to build metrics exporter tokio runtime");

    std::thread::spawn(move || {
        rt.spawn(server);
        rt.shutdown_on_idle().wait().unwrap();
    });
}
```

client/prometheus/Cargo.toml
```toml
[package]
name = "substrate-prometheus"
version = "2.0.0"
authors = ["Parity Technologies <admin@parity.io>"]
description = "prometheus utils"
edition = "2018"

[dependencies]
hyper = "0.12"
lazy_static = "1.0"
log = "0.4"
prometheus = { version = "0.7", features = ["nightly", "process"]}
tokio = "0.1"

[dev-dependencies]
reqwest = "0.9"
```
client/service/Cargo.toml
```toml
....
promet = { package = "substrate-prometheus", path="../../core/prometheus"}
....
```
client/service/src/builder.rs
```rust

				.....
				let _ = to_spawn_tx.unbounded_send(Box::new(future
				.select(exit.clone())
				.then(|_| Ok(()))));
			telemetry
		});
----------------
match config.prometheus_endpoint {
			None => (), 
			Some(x) => {let _prometheus = promet::init_prometheus(x);}	
		}
		
-------------------		
		Ok(NewService {
			client,
			network,
				.....
```
substrate/Cargo.toml
```toml
[workspace]
members = [
	"client/prometheus",
....
```

### List of available metrics


Consensus metrics, namespace: `substrate`

| **Name**                                | **Type**  | **Tags** | **Description**                                                 |
|-----------------------------------------|-----------|----------|-----------------------------------------------------------------|
| consensus_height                        | Gauge     |          | Height of the chain                                             |
| consensus_failure                       | counter   | height   | Consensus failure                                               |
| consensus_validators                    | Gauge     |          | Number of validators                                            |
| consensus_validators_power              | Gauge     |          | Total voting power of all validators                            |
| consensus_missing_validators            | Gauge     |          | Number of validators who did not sign                           |
| consensus_missing_validators_power      | Gauge     |          | Total voting power of the missing validators                    |
| consensus_byzantine_validators          | Gauge     |          | Number of validators who tried to double sign                   |
| consensus_byzantine_validators_power    | Gauge     |          | Total voting power of the byzantine validators                  |
| consensus_block_interval_seconds        | Histogram |          | Time between this and last block (Block.Header.Time) in seconds |
| consensus_rounds                        | Gauge     |          | Number of rounds                                                |
| consensus_num_txs                       | Gauge     |          | Number of transactions                                          |
| consensus_block_parts                   | counter   | peer_id | number of blockparts transmitted by peer                        |
| consensus_latest_block_height           | gauge     |          | /status sync_info number                                       |
| consensus_fast_syncing                  | gauge     |          | either 0 (not fast syncing) or 1 (syncing)                      |
| consensus_total_txs                     | Gauge     |          | Total number of transactions committed                          |
| consensus_block_size_bytes              | Gauge     |          | Block size in bytes                                             |
| p2p_peers                               | Gauge     |          | Number of peers node's connected to                             |
| p2p_peer_receive_bytes_total            | counter   | peer_id | number of bytes received from a given peer                      |
| p2p_peer_send_bytes_total               | counter   | peer_id | number of bytes sent to a given peer                            |
| p2p_peer_pending_send_bytes             | gauge     | peer_id | number of pending bytes to be sent to a given peer              |
| p2p_num_txs                             | gauge     | peer_id | number of transactions submitted by each peer_id               |
| mempool_size                            | Gauge     |          | Number of uncommitted transactions                              |
| mempool_tx_size_bytes                   | histogram |          | transaction sizes in bytes                                      |
| mempool_failed_txs                      | counter   |          | number of failed transactions                                   |
| mempool_recheck_times                   | counter   |          | number of transactions rechecked in the mempool                 |
| state_block_processing_time             | histogram |          | time between BeginBlock and EndBlock in ms                      |
| state_recheck_time                      | histogram |          | time cost on recheck in ms                      |
| state_app_hash_conflict                 | count     | proposer, height | App hash conflict error                      |



## Start Prometheus

### Edit Prometheus config file

You can visit [prometheus.yml](https://github.com/prometheus/prometheus/blob/master/documentation/examples/prometheus.yml) to download default `prometheus.yml`.

Then edit `prometheus.yml` and add `jobs` :

```yaml
      - job_name: kusama
          static_configs:
          - targets: ['localhost:33333']
            labels:
              instance: local-validator
```

> Note：value of targets is ip:port which used by substrate monitor 

### Start Prometheus

```bash
docker run -d --name=prometheus -p 9090:9090 -v ~/volumes/prometheus:/etc/prometheus prom/prometheus
```

> The above example, you can save `prometheus.yml` at `~/volumes/prometheus` on your host machine

You can visit `http://localhost:9090` to see prometheus data.

## Start Grafana

```
docker run -d --name=grafana -p 3000:3000 grafana/grafana
```

You can visit `http://localhost:3000/` to open grafana and create your own dashboard.

> Tips: The default username and password are both admin. We strongly recommend immediately changing your username & password after login