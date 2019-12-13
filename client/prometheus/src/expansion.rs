pub use crate::*;

use sp_runtime::traits::{NumberFor, Block as BlockT};
use fg_primitives::{AuthorityId,AuthoritySignature};
use sp_core::{  crypto::{ Ss58Codec }};
use sysinfo::{NetworkExt, System, SystemExt};

type Blockcast<Block> = grandpa::CatchUp<
	<Block as BlockT>::Hash,
	NumberFor<Block>,
	AuthoritySignature,
	AuthorityId,
>;

pub struct Message<Block: BlockT> {
	/// The compact commit message.
	pub message: Blockcast<Block>,
}

pub fn full_message_metrics<Block: BlockT>(
    message: &Blockcast<Block>,
    authorities: Vec<AuthorityId>
)
{

    //let block_number = &message.base_number.clone().saturated_into().to_string();

    let authorityid_list  = authorities.iter();
    for authorityid in authorityid_list{
        let mut labels = std::collections::HashMap::new();
        let mut _authorityid = &authorityid.clone().to_ss58check();
        labels.insert("validator_address", _authorityid as &str);
        //labels.insert("block_num", block_number as &str);
        metrics::set_vecgauge(&metrics::VALIDATOR_SIGN_PREVOTE ,&labels, 0);
        metrics::set_vecgauge(&metrics::VALIDATOR_SIGN_PRECOMMIT ,&labels, 0);
    };

    let prevote_list = message.prevotes.iter();
    for prevoteid in prevote_list{
        let mut labels = std::collections::HashMap::new();
        let mut _prevoteid = &prevoteid.id.clone().to_ss58check();
        labels.insert("validator_address", _prevoteid as &str);
        //labels.insert("block_num", block_number as &str);
        metrics::set_vecgauge(&metrics::VALIDATOR_SIGN_PREVOTE ,&labels, 1);
    };

    let precommit_list = message.precommits.iter();
    for precommitid in precommit_list{
        let mut labels = std::collections::HashMap::new();
        let mut _precommitid = &precommitid.id.clone().to_ss58check();
        labels.insert("validator_address", _precommitid as &str);
        //labels.insert("block_num", block_number as &str);
        metrics::set_vecgauge(&metrics::VALIDATOR_SIGN_PRECOMMIT ,&labels, 1);
    };
}

pub fn resource_metrics(){
    let mut sys = System::new();

    // We display the disks:
    println!("=> disk list:");
    for disk in sys.get_disks() {
    println!("{:?}", disk);
    }

    // Network data:
    println!("input data : {} B", sys.get_network().get_income());
    println!("output data: {} B", sys.get_network().get_outcome());

    // Components temperature:
    for component in sys.get_components_list() {
        println!("{:?}", component);
    }

    // Memory information:
    println!("total memory: {} kB", sys.get_total_memory());
    println!("used memory : {} kB", sys.get_used_memory());
    println!("total swap  : {} kB", sys.get_total_swap());
    println!("used swap   : {} kB", sys.get_used_swap());

    // Number of processors
    println!("NB processors: {}", sys.get_processor_list().len());
    println!("uptime: {}", sys.get_uptime());
    // To refresh all system information:
    sys.refresh_network();
    sys.refresh_all();
}