pub use crate::*;

use sp_runtime::traits::{NumberFor, Block as BlockT};
use fg_primitives::{AuthorityId,AuthoritySignature};
use sp_core::{  crypto::{ Ss58Codec }};

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