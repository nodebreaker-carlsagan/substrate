pub use crate::*;

use sp_runtime::traits::{NumberFor, Block as BlockT};
use fg_primitives::{AuthorityId,AuthoritySignature};
//sp_finality_grandpa::app::Signature, sp_finality_grandpa::app::Public is to_raw_vec()
use sp_core::{ crypto::{ Public }};


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
    let _block_num = &message.base_number.clone();
    let _precommets =  &message.precommits[0].id.clone().to_raw_vec();
    //alloc::vec::Vec<finality_grandpa::SignedPrecommit<primitive_types::H256, u32, sp_finality_grandpa::app::Signature, sp_finality_grandpa::app::Public>>
    let _prevotes = &message.prevotes[0].id.clone().to_raw_vec(); 
    //alloc::vec::Vec<finality_grandpa::SignedPrevote<primitive_types::H256, u32, sp_finality_grandpa::app::Signature, sp_finality_grandpa::app::Public>>
    let _authorityid  = authorities[0].clone().to_raw_vec();
    //alloc::vec::Vec<sp_finality_grandpa::app::Public>
    println!("{:?}",_block_num);
    println!("{:?}",_precommets);
    println!("{:?}",_prevotes);
    println!("{:?}",_authorityid);
}
    //with fn
    //expansion::full_message_metrics(self.current_authorities.clone());
    //
    //let mut labels2 = std::collections::HashMap::new();
    //labels2.insert("validator_address","fcp1zcjduepqwt4wkpqy9kgg67kmp4wgp7azt7a2h928sac2p3uwnq6vz9vw0h8qqap9sd");
	//labels2.insert("block_num","1");
    //metrics::set_vecgauge(&metrics::VALIDATOR_SIGN ,&labels2,0);