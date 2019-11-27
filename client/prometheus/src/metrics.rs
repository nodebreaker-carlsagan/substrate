pub use crate::*;


pub fn try_create_int_gauge(name: &str, help: &str) -> Result<IntGauge> {
    let opts = Opts::new(name, help);
    let gauge = IntGauge::with_opts(opts)?;
    prometheus::register(Box::new(gauge.clone()))?;
    Ok(gauge)
}

pub fn try_create_histogram(name: &str, help: &str) -> Result<Histogram> {
    let opts = HistogramOpts::new(name, help);
    let histogram = Histogram::with_opts(opts)?;
    prometheus::register(Box::new(histogram.clone()))?;
    Ok(histogram)
}

pub fn try_create_counter(name: &str, help: &str) -> Result<IntCounter> {
    let opts = CounterOpts::new(name, help);
    let counter = IntCounter::with_opts(opts)?;
    prometheus::registrer(Box::new(counter.clone()))?;
    Ok(counter)
}

pub fn set_gauge(gauge: &Result<IntGauge>, value: u64) {
    if let Ok(gauge) = gauge {
        gauge.set(value as i64);
    }
    
}

pub fn set_histogram(histogram: &Result<Histogram>, value: f64) {
    if let Ok(histogram) = histogram {
        histogram.observe(value)
    }
}

pub fn set_counter(counter: &Result<IntCounter>, value: u64) {
    if let Ok(counter) = counter {
        counter.set(value as i64);
    }
}

lazy_static! {
    pub static ref FINALITY_HEIGHT: Result<IntGauge> = try_create_int_gauge(
        "consensus_finality_block_height_number",
        "block is finality HEIGHT"

    );
    pub static ref BEST_HEIGHT: Result<IntGauge> = try_create_int_gauge(
        "consensus_best_block_height_number",
        "block is best HEIGHT"
    );
    pub static ref VALIDATORS_POWER: Result<IntGauge> = try_create_int_gauge(
        "consensus_best_block_height_number",
        "Total voting power of all validators"
    );
    pub static ref MISSING_VALIDATORS: Result<IntGauge> = try_create_int_gauge(
        "consensus_missing_validators",
        "Number of validators who did not sign"
    );
    pub static ref MISSING_VALIDATORS_POWER: Result<IntGauge> = try_create_int_gauge(
        "consensus_missing_validators_power",
        "Total voting power of the missing validators"
    );
    pub static ref BYZANTINE_VALIDATORS: Result<IntGauge> = try_create_int_gauge(
        "consensus_byzantine_validators",
        "Number of validators who tried to double sign"
    );
    pub static ref BYZANTINE_VALIDATORS_POWER: Result<IntGauge> = try_create_int_gauge(
        "consensus_byzantine_validators_power",
        "Total voting power of the byzantine validators"
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
    //net_status.average_download_per_sec
    pub static ref P2P_NODE_DOWNLOAD: Result<IntGauge> = try_create_int_gauge(
        "p2p_peer_receive_byte_per_sec",
        "p2p_node_download_per_sec_byte"
    );
    pub static ref P2P_NODE_UPLOAD: Result<IntGauge> = try_create_int_gauge(
        "p2p_peer_send_byte_per_sec",
        "p2p_node_upload_per_sec_byte"
    );
}


