pub use crate::*;

pub fn try_create_int_gauge(name: &str, help: &str) -> Result<IntGauge> {
    let opts = Opts::new(name, help);
    let gauge = IntGauge::with_opts(opts)?;
    prometheus::register(Box::new(gauge.clone()))?;
    Ok(gauge)
}

pub fn set_gauge(gauge: &Result<IntGauge>, value: u64) {
    if let Ok(gauge) = gauge {
        gauge.set(value as i64);
    }
}

lazy_static! {
    pub static ref FINALITY_HEIGHT: Result<IntGauge> = try_create_int_gauge(
        "finality_block_height_number",
        "block is finality HEIGHT"
    );
    pub static ref BEST_HEIGHT: Result<IntGauge> = try_create_int_gauge(
        "best_block_height_number",
        "block is best HEIGHT"
    );
    pub static ref PEERS_NUM: Result<IntGauge> = try_create_int_gauge(
        "peers_number",
        "network gosip peers number"
    );
}