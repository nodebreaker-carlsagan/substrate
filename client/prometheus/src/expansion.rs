pub use crate::*;

//use sp_runtime::traits::{NumberFor, Block as BlockT};
use fg_primitives::{AuthorityId,AuthoritySignature};
use sp_core::{
    crypto::{ Public } ,H256
};
use grandpa::{
    SignedPrevote, SignedPrecommit,Prevote
};


pub fn full_message_metrics(
    //prevote_list: &Vec<T>,
    prevote_list: &Vec<SignedPrevote< Prevote<H256,u32>, u32,AuthoritySignature, AuthorityId>>,
    precommet_list: &Vec<SignedPrecommit<H256, u32, AuthoritySignature, AuthorityId>>,
    base_num: u32,
    authorities: AuthorityId
)
{
    let _block_num = base_num;
    let _precommets =  precommet_list.clone();
    //alloc::vec::Vec<finality_grandpa::SignedPrecommit<primitive_types::H256, u32, sp_finality_grandpa::app::Signature, sp_finality_grandpa::app::Public>>
    let _prevotes = prevote_list.clone(); 
    //alloc::vec::Vec<finality_grandpa::SignedPrevote<primitive_types::H256, u32, sp_finality_grandpa::app::Signature, sp_finality_grandpa::app::Public>>
    let _authorityid  = authorities.clone().to_raw_vec();
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