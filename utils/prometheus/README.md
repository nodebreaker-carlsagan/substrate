# Substrate Prometheus Node Exporter
![grants](./photo_2019-12-13_16-32-53.jpg)
## Introduction

Prometheus is one of the most widely used monitoring tool for managing high availability services supported by [Cloud Native Computing Foundation](https://www.cncf.io/). By providing Prometheus metrics in Substrate, node operators can easily adopt widely used display/alert tools such as Grafana and Alertmanager without setting-up/operating external Prometheus push gateways (which is an antipattern in the first place) through RPC connections. Easy access to such monitoring tools will benefit parachain developers/operators and validators to have much higher availability of their services.

## Table of Contents

Hack Prometheus in Substrate
 - Prometheus primer
 - CLI Config
 - Metrics Add
 - expansion Add 

Metrics
 - List of available metrics

Start Prometheus
 - Install prometheus
 - Edit Prometheus config file
 - Start Prometheus

Start Grafana
 - Install Grafana

## Substrate Dev hack
### Prometheus primer

Here is the entry point of prometheus core module in Parity Substrate.

In existing sources, refer to the Grafana source due to the issue of the wasm.

utils/prometheus/src/lib.rs
```rust
#[macro_use]
extern crate lazy_static;
use futures_util::{FutureExt,future::{Future}};
use hyper::http::StatusCode;
use hyper::Server;
use hyper::{Body, Request, Response, service::{service_fn, make_service_fn}};
pub use prometheus::{Encoder, HistogramOpts, Opts, TextEncoder};
pub use prometheus::{Histogram, IntCounter, IntGauge};
pub use sp_runtime::traits::SaturatedConversion;
use std::net::SocketAddr;

pub mod metrics;
pub mod expansion;

#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
	/// Hyper internal error.
	Hyper(hyper::Error),
	/// Http request error.
	Http(hyper::http::Error),
	/// i/o error.
	Io(std::io::Error)
}
impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Error::Hyper(error) => Some(error),
			Error::Http(error) => Some(error),
			Error::Io(error) => Some(error)
		}
	}
}

async fn request_metrics(req: Request<Body>) -> Result<Response<Body>, Error> {
  if req.uri().path() == "/metrics" {
	expansion::resource_metrics();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    Response::builder()
      .status(StatusCode::OK)
      .header("Content-Type", encoder.format_type())
      .body(Body::from(buffer))
      .map_err(Error::Http)
  } else {
    Response::builder()
      .status(StatusCode::NOT_FOUND)
      .body(Body::from("Not found."))
      .map_err(Error::Http)
  }
  
}

#[derive(Clone)]
pub struct Executor;

#[cfg(not(target_os = "unknown"))]
impl<T> hyper::rt::Executor<T> for Executor
	where
		T: Future + Send + 'static,
		T::Output: Send + 'static,
{
	fn execute(&self, future: T) {
		async_std::task::spawn(future);
	}
}
/// Initializes the metrics context, and starts an HTTP server
/// to serve metrics.
#[cfg(not(target_os = "unknown"))]
pub  async fn init_prometheus(mut prometheus_addr: SocketAddr) -> Result<(), Error>{
  use async_std::{net, io};
  use grafana_data_source::networking::Incoming;

	let listener = loop {
		let listener = net::TcpListener::bind(&prometheus_addr).await;
		match listener {
			Ok(listener) => {
				log::info!("Prometheus server started at {}", prometheus_addr);
				break listener
			},
			Err(err) => match err.kind() {
				io::ErrorKind::AddrInUse | io::ErrorKind::PermissionDenied if prometheus_addr.port() != 0 => {
					log::warn!(
						"Prometheus server to already {} port.", prometheus_addr.port()
					);
					prometheus_addr.set_port(0);
					continue;
				},
        _ => return Err(err.into())
      }
		}
	};
  let service = make_service_fn(|_| {
		async {
			Ok::<_, Error>(service_fn(request_metrics))
		}
	});


	let _server = Server::builder(Incoming(listener.incoming()))
		.executor(Executor)
		.serve(service)
    .boxed();
  
  
	let result = _server.await.map_err(Into::into);

	result
}

#[cfg(target_os = "unknown")]
pub async fn init_prometheus(_: SocketAddr) -> Result<(), Error> {
	Ok(())
}


#[macro_export]
macro_rules! prometheus_gauge(
  ($($metric:expr => $value:expr),*) => {
    use $crate::{metrics::*};
    $(
        metrics::set_gauge(&$metric, $value);
    )*
  }
);

#[macro_export]
macro_rules! prometheus_histogram(
  ($($metric:expr => $value:expr),*) => {
    use $crate::{metrics::*};
    $(
        metrics::set_histogram(&$metric, $value);
    )*
  }
);


```



Here is the dependancies of the module.	
utils/prometheus/Cargo.toml
```toml
[package]
name = "sc-prometheus"
version = "2.0.0"
authors = ["Parity Technologies <admin@parity.io>"]
description = "prometheus utils"
edition = "2018"

[dependencies]
sysinfo = "0.10.2"
hyper = { version = "0.13.1", default-features = false, features = ["stream"] }
lazy_static = "1.4"
log = "0.4.8"
tokio = "0.2"
prometheus = { version = "0.7", features = ["nightly", "process"]}
tokio = "0.2"
futures-util = { version = "0.3.1", default-features = false, features = ["io"] }
sp-runtime = { package = "sp-runtime",path = "../../primitives/runtime" }
fg-primitives = { package = "sp-finality-grandpa", path = "../../primitives/finality-grandpa" }
sp-core = { package = "sp-core",path = "../../primitives/core" }
grandpa = { package = "finality-grandpa", version = "0.10.1", features = ["derive-codec"] }
derive_more = "0.99"



[target.'cfg(not(target_os = "unknown"))'.dependencies]
async-std = { version = "1.0.1", features = ["unstable"] }
```

**Abbreviation of the package in service manager of parity substrate**	
client/service/Cargo.toml
```toml
[dependencies]
sc-prometheus = { package = "sc-prometheus", path="../../utils/prometheus"}
```

**Metrics builder as same as substrate-telemetry**	
client/service/src/builder.rsL1271 , L1112
```rust
use sc_prometheus::prometheus_gauge;
...
			telemetry!(
				SUBSTRATE_INFO;
				"system.interval";
				"peers" => num_peers,
				"height" => best_number,
				"best" => ?best_hash,
				"txcount" => txpool_status.ready,
				"cpu" => cpu_usage,
				"memory" => memory,
				"finalized_height" => finalized_number,
				"finalized_hash" => ?info.chain.finalized_hash,
				"bandwidth_download" => bandwidth_download,
				"bandwidth_upload" => bandwidth_upload,
				"used_state_cache_size" => used_state_cache_size,
			);
			prometheus_gauge!(
				STATE_CACHE_SIZE => used_state_cache_size as u64,
				NODE_MEMORY => memory as u64,
				NODE_CPU => cpu_usage as u64,
				TX_COUNT => txpool_status.ready as u64,
				FINALITY_HEIGHT => finalized_number as u64,
				BEST_HEIGHT => best_number as u64,
				P2P_PEERS_NUM => num_peers as u64,
				P2P_NODE_DOWNLOAD => net_status.average_download_per_sec as u64,
				P2P_NODE_UPLOAD => net_status.average_upload_per_sec as u64
			  );
			let _ = record_metrics!(
				"peers" => num_peers,
				"height" => best_number,
				"txcount" => txpool_status.ready,
				"cpu" => cpu_usage,
				"memory" => memory,
				"finalized_height" => finalized_number,
				"bandwidth_download" => bandwidth_download,
				"bandwidth_upload" => bandwidth_upload,
				"used_state_cache_size" => used_state_cache_size,
			  );
			Ok(())
		}).select(exit.clone().map(Ok).compat()).then(|_| Ok(()));
		let _ = to_spawn_tx.unbounded_send(Box::new(tel_task));

...
		// prometheus init
		if let Some(port) = config.prometheus_port {
			let future = select(
					sc_prometheus::init_prometheus(port).boxed()
					,exit.clone()
				).map(|either| match either {
					Either::Left((result, _)) => result.map_err(|_| ()),
					Either::Right(_) => Ok(())
				}).compat();
				let _ = to_spawn_tx.unbounded_send(Box::new(future));
		}
		// Grafana data source
		if let Some(port) = config.grafana_port {
			let future = select(
				grafana_data_source::run_server(port).boxed(),
				exit.clone()
			).map(|either| match either {
				Either::Left((result, _)) => result.map_err(|_| ()),
				Either::Right(_) => Ok(())
			}).compat();

			let _ = to_spawn_tx.unbounded_send(Box::new(future));
    }

		// Instrumentation
		if let Some(tracing_targets) = config.tracing_targets.as_ref() {
			let subscriber = sc_tracing::ProfilingSubscriber::new(
				config.tracing_receiver, tracing_targets
			);
			match tracing::subscriber::set_global_default(subscriber) {
				Ok(_) => (),
				Err(e) => error!(target: "tracing", "Unable to set global default subscriber {}", e),
			}
		}


```
substrate/Cargo.toml
```toml
[workspace]
members = [
	"utils/prometheus",
```
### CLI Config
client/cli/src/lib.rs
```rust
fn crate_run_node_config{
...
	let prometheus_interface: &str = if cli.prometheus_external { "0.0.0.0" }
...
		// Override prometheus
	if cli.prometheus_external {
			config.prometheus_port = Some(
		parse_address(&format!("{}:{}", prometheus_interface, 33333), cli.prometheus_port)?
	)}
}
```

client/cli/src/params.rs
```rust
pub struct RunCmd{
...
	/// Prometheus exporter TCP port.
	#[structopt(long = "prometheus-port", value_name = "PORT")]
	pub prometheus_port: Option<u16>,
	/// Prometheus exporter on/off external".
	#[structopt(long = "prometheus-external")]
	pub prometheus_external: bool,
...
}
```
client/service/src/config.rs
```rust
#[derive(Clone)]
pub struct Configuration<C, G, E = NoExtension> {
    ...
	/// Prometheus Port.`None` if disabled and port 33333 by default.
	pub prometheus_port: Option<SocketAddr>,
    ...
}
impl<C, G, E> Configuration<C, G, E> where
	C: Default,
	G: RuntimeGenesis,
	E: Extension,
{
	/// Create default config for given chain spec.
	pub fn default_with_spec(chain_spec: ChainSpec<G, E>) -> Self {
		let mut configuration = Configuration {
            ...
            prometheus_prot: None,
            ...
		};
		configuration.network.boot_nodes = configuration.chain_spec.boot_nodes().to_vec();

		configuration.telemetry_endpoints = configuration.chain_spec.telemetry_endpoints().clone();

		configuration
	}
```




### Metrics Add 
ex) consensus_FINALITY_HEIGHT

utils/prometheus/src/metrics.rs

```rust
pub use crate::*;

/// Gauge type metrics generation function
pub fn try_create_int_gauge(name: &str, help: &str) -> Result<IntGauge> {
    let opts = Opts::new(name, help);
    let gauge = IntGauge::with_opts(opts)?;
    prometheus::register(Box::new(gauge.clone()))?;
    Ok(gauge)
}

///Gauge Metrics a value in injection.
pub fn set_gauge(gauge: &Result<IntGauge>, value: u64) {
    if let Ok(gauge) = gauge {
        gauge.set(value as i64);
    }
}

///All of the metrics in the prometheus are managed by the lazy_static.
lazy_static! {
    pub static ref FINALITY_HEIGHT: Result<IntGauge> = try_create_int_gauge(
        "consensus_finality_block_height_number",
        "block is finality HEIGHT"
    );
}
```
client/service/Cargo.toml
```rust
...
sc-prometheus = { package = "sc-prometheus", path="../../utils/prometheus"}
...
```
utils/service/src/builder.rs
```rust
.....
use sc-prometheus::{prometheus_gauge};
.....
		let tel_task = state_rx.for_each(move |(net_status, _)| {
			let info = client_.info();
			let best_number = info.chain.best_number.saturated_into::<u64>();
			let best_hash = info.chain.best_hash;
			let num_peers = net_status.num_connected_peers;
			let txpool_status = transaction_pool_.status();
			let finalized_number: u64 = info.chain.finalized_number.saturated_into::<u64>();
			let bandwidth_download = net_status.average_download_per_sec;
			let bandwidth_upload = net_status.average_upload_per_sec;

			let used_state_cache_size = match info.used_state_cache_size {
				Some(size) => size,
				None => 0,
			};

			// get cpu usage and memory usage of this process
			let (cpu_usage, memory) = if let Some(self_pid) = self_pid {
				if sys.refresh_process(self_pid) {
					let proc = sys.get_process(self_pid)
						.expect("Above refresh_process succeeds, this should be Some(), qed");
					(proc.cpu_usage(), proc.memory())
				} else { (0.0, 0) }
			} else { (0.0, 0) };

			telemetry!(
				SUBSTRATE_INFO;
				"system.interval";
				"peers" => num_peers,
				"height" => best_number,
				"best" => ?best_hash,
				"txcount" => txpool_status.ready,
				"cpu" => cpu_usage,
				"memory" => memory,
				"finalized_height" => finalized_number,
				"finalized_hash" => ?info.chain.finalized_hash,
				"bandwidth_download" => bandwidth_download,
				"bandwidth_upload" => bandwidth_upload,
				"used_state_cache_size" => used_state_cache_size,
			);

			prometheus_gauge!(
				  STATE_CACHE_SIZE => used_state_cache_size as u64,
				  NODE_MEMORY => memory as u64,
				  NODE_CPU => cpu_usage as u64,
				  TX_COUNT => txpool_status.ready as u64,
				  FINALITY_HEIGHT => finalized_number as u64,
				  BEST_HEIGHT => best_number as u64,
				  P2P_PEERS_NUM => num_peers as u64,
				  P2P_NODE_DOWNLOAD => net_status.average_download_per_sec as u64,
				  P2P_NODE_UPLOAD => net_status.average_upload_per_sec as u64
				);
.....
```

### Expansion Metrics

utils/prometheus/src/expansion.rs
```rust
pub use crate::*;

use fg_primitives::{AuthorityId, AuthoritySignature};
use sp_core::crypto::Ss58Codec;
use sp_runtime::traits::{Block as BlockT, NumberFor};
use sysinfo::{NetworkExt, System, SystemExt, DiskExt, ProcessorExt};
use std::{thread,time};

type Blockcast<Block> =
    grandpa::CatchUp<<Block as BlockT>::Hash, NumberFor<Block>, AuthoritySignature, AuthorityId>;

pub struct Message<Block: BlockT> {
    /// The compact commit message.
    pub message: Blockcast<Block>,
}

pub fn full_message_metrics<Block: BlockT>(
    message: &Blockcast<Block>,
    authorities: Vec<AuthorityId>,
) {
    //let block_number = &message.base_number.clone().saturated_into().to_string();

    let authorityid_list = authorities.iter();
    for authorityid in authorityid_list {
        let mut labels = std::collections::HashMap::new();
        let mut _authorityid = &authorityid.clone().to_ss58check();
        labels.insert("validator_address", _authorityid as &str);
        //labels.insert("block_num", block_number as &str);
        metrics::set_vecgauge(&metrics::VALIDATOR_SIGN_PREVOTE, &labels, 0);
        metrics::set_vecgauge(&metrics::VALIDATOR_SIGN_PRECOMMIT, &labels, 0);
    }

    let prevote_list = message.prevotes.iter();
    for prevoteid in prevote_list {
        let mut labels = std::collections::HashMap::new();
        let mut _prevoteid = &prevoteid.id.clone().to_ss58check();
        labels.insert("validator_address", _prevoteid as &str);
        //labels.insert("block_num", block_number as &str);
        metrics::set_vecgauge(&metrics::VALIDATOR_SIGN_PREVOTE, &labels, 1);
    }

    let precommit_list = message.precommits.iter();
    for precommitid in precommit_list {
        let mut labels = std::collections::HashMap::new();
        let mut _precommitid = &precommitid.id.clone().to_ss58check();
        labels.insert("validator_address", _precommitid as &str);
        //labels.insert("block_num", block_number as &str);
        metrics::set_vecgauge(&metrics::VALIDATOR_SIGN_PRECOMMIT, &labels, 1);
    }
}

pub fn resource_metrics() {
    let mut sys = System::new();
    let ten_millis = time::Duration::from_millis(500);
    thread::sleep(ten_millis);
    sys.refresh_all();
    let cpu_use = (sys.get_processor_list()[0].get_cpu_usage() * 100.0).round() as u64;
    metrics::set_gauge(&metrics::RESOURCE_CPU_USE, cpu_use);
    println!("use: {:?}", cpu_use);

    for disk in sys.get_disks() {
        let mount_point = disk.get_mount_point().to_str().unwrap();
        let used_disk = disk.get_total_space() - disk.get_available_space();
        let mut labels = std::collections::HashMap::new();
        labels.insert("mount_point", mount_point);
        metrics::set_vecgauge(&metrics::RESOURCE_DISK_USE, &labels, used_disk as u64);
    }
    // Prometheus usage
    metrics::set_gauge(&metrics::RESOURCE_RECEIVE_BYTES, sys.get_network().get_income() as u64);
    metrics::set_gauge(&metrics::RESOURCE_SENT_BYTES, sys.get_network().get_outcome() as u64);
    metrics::set_gauge(&metrics::RESOURCE_RAM_USE , sys.get_used_memory() as u64);
    metrics::set_gauge(&metrics::RESOURCE_SWAP_USE , sys.get_used_swap() as u64);
}

```

client/finality-grandpa/src/communication/gossip.rs:L813
```rust
+ use sc_prometheus::{expansion};

+ expansion::full_message_metrics::<Block>(&full.message.clone(),self.authorities.clone());
```
client/finality-grandpa/Cargo.toml
```rust
+ sc-prometheus = { path = "../../utils/prometheus" }
```


## Metrics

substrate can report and serve the Prometheus metrics, which in their turn can be consumed by Prometheus collector(s).

This functionality is disabled by default.

To enable the Prometheus metrics, set in your cli command (--prometheus-addr,--prometheus-port ). 
Metrics will be served under /metrics on 33333 port by default.

### List of available metrics


Consensus metrics, namespace: `substrate`

| **Name**                               | **Type**  | **Tags** | **Description**                                                 |
| -------------------------------------- | --------- | -------- | --------------------------------------------------------------- |
| consensus_finality_block_height_number | IntGauge  |          | finality Height of the chain                                    |
| consensus_best_block_height_number     | IntGauge  |          | best Height of the chain                                        |
| consensus_target_syn_number            | IntGauge  |          | syning Height target number                                     |
| consensus_num_txs                      | Gauge     |          | Number of transactions                                          |
| consensus_node_memory                  | IntGauge  |          | Node's primary memory                                           |
| consensus_node_cpu                     | IntGauge  |          | Node's cpu load                                                 |
| consensus_state_cache_size             | IntGauge  |          | used state cache size                             			  |
| p2p_peers_number                       | IntGauge  |          | Number of peers node's connected to                             |
| p2p_peer_receive_bytes_per_sec         | IntGauge  |          | number of bytes received from a given peer                      |
| p2p_peer_send_bytes_per_sec            | IntGauge  |          | number of bytes sent to a given peer                            |
| mempool_size                           | IntGauge  |          | Number of uncommitted transactions                              |
| Resource_receive_bytes_per_sec | IntGauge  |          | Operating System of bytes received                              |
| Resource_send_bytes_per_sec    | IntGauge  |          | Operating System of bytes sent                                  |
| Resource_cpu_use              | IntGauge  |          | Operating System cpu load                                       |
| Resource_disk_use             | IntGauge  |          | Operating System disk use                                      |
| Resource_memory_use            | IntGauge  |          | Operating System memory use                                      |
| Resource_swap_use              | IntGauge  |          | Operating System swap memory use                                      |
| validator_sign_prevote        | IntGauge  | validator addr | validator sign vote list                               	  |
| validator_sign_precommit      | IntGauge  | validator addr | validator sign commit list                                |


## Start Prometheus
### Install prometheus

https://prometheus.io/download/
```bash
wget <download URL>
tar -zxvf <prometheus tar file>
```

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
cd <prometheus file>
./prometheus
```

> The above example, you can save `prometheus.yml` at `~/volumes/prometheus` on your host machine

You can visit `http://localhost:9090` to see prometheus data.



## Start Grafana
### Install Grafana
https://grafana.com/docs/installation/debian/

```bash
apt-get install -y software-properties-common
sudo add-apt-repository "deb https://packages.grafana.com/oss/deb stable main"
wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -
sudo apt-get update
sudo apt-get install grafana
sudo service grafana-server start
./prometheus
```

You can visit `http://localhost:3000/` to open grafana and create your own dashboard.

> Tips: The default username and password are both admin. We strongly recommend immediately changing your username & password after login

### Seting Grafana

Default ID:PW is admin.

Samples(dashboard-export.json import), Prometheus Node IPs must be set directly from the panel.

ex)
![grants](./dashboard.PNG)