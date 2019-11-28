pub use crate::*;

use fg_primitives::{AuthorityId, AuthoritySignature};
use sp_core::crypto::Ss58Codec;
use sp_runtime::traits::{Block as BlockT, NumberFor};
use sysinfo::{NetworkExt, System, SystemExt, DiskExt, ProcessorExt};
use std::{thread,time};


//To collect and configure the value from gossip, cast the block type as it is.
type Blockcast<Block> =
    grandpa::CatchUp<<Block as BlockT>::Hash, NumberFor<Block>, AuthoritySignature, AuthorityId>;

pub struct Message<Block: BlockT> {
    /// The compact commit message.
    pub message: Blockcast<Block>,
}
//Configure metrics that are overwritten
//First write the authored list, then write prevote,precommit,and cover each ss58 address.
//Note: It is possible to receive configuration by block, but it has not been implemented. Because there's so much information, Prometheus stops.
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


//dragged resource function from std,
//Because of the cpu usage, a slight delay is required and time function is applied
pub fn resource_metrics() {
    let mut sys = System::new();
    let ten_millis = time::Duration::from_millis(500);
    thread::sleep(ten_millis);
    sys.refresh_all();
    let cpu_use = (sys.get_processor_list()[0].get_cpu_usage() * 100.0).round() as u64;
    metrics::set_gauge(&metrics::RESOURCE_CPU_USE, cpu_use);

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
