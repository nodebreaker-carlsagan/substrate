pub use crate::*;

/// Gauge type metrics generation function
pub fn try_create_int_gaugevec(name: &str, help: &str, tag: &[&str]) -> Result<IntGaugeVec> {
    let opts = Opts::new(name, help);
    let gaugevec = IntGaugeVec::new(opts, tag)?;
    prometheus::register(Box::new(gaugevec.clone()))?;
    Ok(gaugevec)
}

/// Gauge type metrics generation function
pub fn try_create_int_gauge(name: &str, help: &str) -> Result<IntGauge> {
    let opts = Opts::new(name, help);
    let gauge = IntGauge::with_opts(opts)?;
    prometheus::register(Box::new(gauge.clone()))?;
    Ok(gauge)
}
/// histogram type metrics generation function
pub fn try_create_histogram(name: &str, help: &str) -> Result<Histogram> {
    let opts = HistogramOpts::new(name, help);
    let histogram = Histogram::with_opts(opts)?;
    prometheus::register(Box::new(histogram.clone()))?;
    Ok(histogram)
}

pub fn set_vecgauge(gaugevec: &Result<IntGaugeVec>, labels: &HashMap<&str, &str>, value: u64) {
    if let Ok(gaugevec) = gaugevec {
        gaugevec.with(&labels).set(value as i64);
    }
}

///Gauge Metrics a value in injection.
pub fn set_gauge(gauge: &Result<IntGauge>, value: u64) {
    if let Ok(gauge) = gauge {
        gauge.set(value as i64);
    }
}
///histogram Metrics a value in injection.
pub fn set_histogram(histogram: &Result<Histogram>, value: f64) {
    if let Ok(histogram) = histogram {
        histogram.observe(value)
    }
}
//All of the metrics in the prometheus are managed by the lazy_static.
lazy_static! {
    pub static ref VALIDATOR_SIGN_PREVOTE: Result<IntGaugeVec> = try_create_int_gaugevec(
        "consensus_validator_block_sign_prevote",
        "block is validator prevote sign",
        //&["validator_address","block_num"]
        &["validator_address"]
    );
    pub static ref VALIDATOR_SIGN_PRECOMMIT: Result<IntGaugeVec> = try_create_int_gaugevec(
        "consensus_validator_block_sign_precommit",
        "block is validator precommit sign",
        //&["validator_address","block_num"]
        &["validator_address"]
    );
    pub static ref FINALITY_HEIGHT: Result<IntGauge> = try_create_int_gauge(
        "consensus_finality_block_height_number",
        "block is finality HEIGHT"

    );
    pub static ref BEST_HEIGHT: Result<IntGauge> = try_create_int_gauge(
        "consensus_best_block_height_number",
        "block is best HEIGHT"
    );

    pub static ref BLOCK_INTERVAL_SECONDS: Result<Histogram> = try_create_histogram(
        "consensus_block_interval_seconds",
        "Time between this and last block(Block.Header.Time) in seconds"
    );
    pub static ref P2P_PEERS_NUM: Result<IntGauge> = try_create_int_gauge(
        "p2p_peers_number",
        "network gosip peers number"
    );
    pub static ref TARGET_NUM: Result<IntGauge> = try_create_int_gauge(
        "consensus_target_syn_number",
        "block syn target number"
    );
    pub static ref TX_COUNT: Result<IntGauge> = try_create_int_gauge(
        "consensus_num_txs",
        "Number of transactions"
    );
    pub static ref NODE_MEMORY: Result<IntGauge> = try_create_int_gauge(
        "consensus_node_memory",
        "node memory"
    );
    pub static ref NODE_CPU: Result<IntGauge> = try_create_int_gauge(
        "consensus_node_cpu",
        "node cpu"
    );
    pub static ref MEMPOOL_SIZE: Result<IntGauge> = try_create_int_gauge(
        "mempool_size",
        "Number of uncommitted transactions"
    );
    pub static ref P2P_NODE_DOWNLOAD: Result<IntGauge> = try_create_int_gauge(
        "p2p_peers_receive_byte_per_sec",
        "p2p_node_download_per_sec_byte"
    );
    pub static ref P2P_NODE_UPLOAD: Result<IntGauge> = try_create_int_gauge(
        "p2p_peers_send_byte_per_sec",
        "p2p_node_upload_per_sec_byte"
    );
    pub static ref RESOURCE_RECEIVE_BYTES: Result<IntGauge> = try_create_int_gauge(
        "resource_receive_bytes_per_sec",
        "Operating system's of bytes received through network card"
    );
    pub static ref RESOURCE_SENT_BYTES: Result<IntGauge> = try_create_int_gauge(
        "resource_send_bytes_per_sec",
        "Operating system's of bytes sent from network card"
    );
    pub static ref RESOURCE_CPU_USE: Result<IntGauge> = try_create_int_gauge(
        "resource_cpu_use",
        "Operating system's whole cpu load"
    );
    pub static ref RESOURCE_RAM_USE: Result<IntGauge> = try_create_int_gauge(
        "resource_cpu_use",
        "Operating system's whole RAM usage"
    );
    pub static ref RESOURCE_SWAP_USE: Result<IntGauge> = try_create_int_gauge(
        "resource_swap_use",
        "Operating system's swap memory use"
    );
}
