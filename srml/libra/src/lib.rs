// Copyright 2017-2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! # Aura Module
//!
//! - [`aura::Trait`](./trait.Trait.html)
//! - [`Module`](./struct.Module.html)
//!
//! ## Overview
//!
//! The Aura module extends Aura consensus by managing offline reporting.
//!
//! ## Interface
//!
//! ### Public Functions
//!
//! - `slot_duration` - Determine the Aura slot-duration based on the Timestamp module configuration.
//!
//! ## Related Modules
//!
//! - [Timestamp](../srml_timestamp/index.html): The Timestamp module is used in Aura to track
//! consensus rounds (via `slots`).
//! - [Consensus](../srml_consensus/index.html): The Consensus module does not relate directly to Aura,
//!  but serves to manage offline reporting by implementing `ProvideInherent` in a similar way.
//!
//! ## References
//!
//! If you're interested in hacking on this module, it is useful to understand the interaction with
//! `substrate/core/inherents/src/lib.rs` and, specifically, the required implementation of
//! [`ProvideInherent`](../substrate_inherents/trait.ProvideInherent.html) and
//! [`ProvideInherentData`](../substrate_inherents/trait.ProvideInherentData.html) to create and check inherents.

#![cfg_attr(not(feature = "std"), no_std)]

pub use timestamp;

use rstd::{result, prelude::*};
use codec::{Encode, Decode};
use support::{
	decl_storage, decl_module, Parameter, storage::StorageValue, traits::{Get, FindAuthor},
	ConsensusEngineId,decl_event,storage::StorageMap,dispatch::Result,
};
use app_crypto::AppPublic;
use sr_primitives::{
	traits::{SaturatedConversion, Saturating, Zero, Member, IsMember}, generic::DigestItem,
};
use timestamp::OnTimestampSet;
#[cfg(feature = "std")]
use timestamp::TimestampInherentData;
use inherents::{RuntimeString, InherentIdentifier, InherentData, ProvideInherent, MakeFatalError};
#[cfg(feature = "std")]
use inherents::{InherentDataProviders, ProvideInherentData};
use substrate_consensus_aura_primitives::{AURA_ENGINE_ID, ConsensusLog, AuthorityIndex};

mod mock;
mod tests;


#[cfg(feature = "std")]
use types::{account_address::AccountAddress,
			transaction::SignedTransaction,
			access_path::AccessPath};
#[cfg(feature = "std")]
use executor::*;
#[cfg(feature = "std")]
use config::config::VMConfig;
#[cfg(feature = "std")]
use vm_runtime::MoveVM;

#[cfg(feature = "std")]
use language_e2e_tests::{
	account::{AccountData,Account},
	common_transactions::{peer_to_peer_txn,create_account_txn,mint_txn},
	executor::FakeExecutor,
	data_store::FakeDataStore,
	data_store::GetHashMap,
	compile::compile_program_with_address,
};
#[cfg(feature = "std")]
use canonical_serialization::*;
#[cfg(feature = "std")]
use serde_json;
#[cfg(feature = "std")]
use std::prelude::v1::Vec;
use std::collections::HashMap;


#[cfg(feature = "std")]
pub type Vechashmap = std::collections::HashMap<AccessPath,Vec<u8>>;


pub trait Trait: timestamp::Trait {
	/// The identifier type for an authority.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as Libra {
		/// The last timestamp.
		LastTimestamp get(last) build(|_| 0.into()): T::Moment;

		pub StoreData get(store_data) : Vec<u8>;

		pub Init get(init) : bool = true;

        pub Libra_Hash_Map get(libra_hash_map): Vec<u8>;
	}
}


decl_event!(
	pub enum Event<T> where <T as system::Trait>::AccountId {
		OfflineWarning(AccountId, u32),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

	    pub fn e2e_libra(origin,txn:Vec<u8>){
	       #[cfg(feature = "std")]
	       Self::execute_libra_transaction(txn);
	    }

        pub fn create_gen_acc(origin){
            #[cfg(feature = "std")]
            Self::create_association_account();
        }

	    fn on_finalize() {
		}

	 }
}

impl<T: Trait> Module<T> {

	#[cfg(feature = "std")]
	fn execute_libra_block() {
		let vm = MoveVM::new(&VMConfig::default());

	}

	#[cfg(feature = "std")]
	fn create_libra_account(){

	}

	#[cfg(feature = "std")]
	fn make_libra_transaction(){

	}

	#[cfg(feature = "std")]
	fn return_a_tx() -> Vec<u8>{
		let sender = AccountData::new(1_000_000, 10);
		let receiver = AccountData::new(100_000, 10);
		let transfer_amount = 1_000;
		let txn = peer_to_peer_txn(sender.account(), receiver.account(), 10, transfer_amount);
		let vec:Vec<u8> = SimpleSerializer::serialize(&txn).unwrap();
		vec
	}

     #[cfg(feature = "std")]
     fn e2e_test(){

		 let mut executor = FakeExecutor::from_genesis_file();

		 let mut data_store = executor.get_data_store();
		 println!("{:?}",data_store);

		 let sender = AccountData::new(1_000_000, 10);
		 let receiver = AccountData::new(100_000, 10);
		 executor.add_account_data(&sender);

		 executor.add_account_data(&receiver);

		 let transfer_amount = 1_000;
		 let txn = peer_to_peer_txn(sender.account(), receiver.account(), 10, transfer_amount);

		 // execute transaction
		 let txns: Vec<SignedTransaction> = vec![txn];
		 //这是执行入口了，此前的操作都需要用户来做
		 let output = executor.execute_block(txns);

		 let mut data_store = executor.get_data_store();
		 println!("{:?}",data_store);
		 // save store_data on substrate
		 //Self::find_store(&executor);

		 println!("{:?}",output);
	 }
/*
        #[cfg(feature = "std")]
        pub fn save_data(executor: &mut FakeExecutor){
            let mut data_store = executor.get_data_store();
            let hashmap = data_store.get_hash_map();

        }
*/

	#[cfg(feature = "std")]
	pub fn execute_libra_transaction(txn:Vec<u8>) -> Result{
		println!("1");
		//deseri_txns
		let txn_de : SignedTransaction =  SimpleDeserializer::deserialize(&txn).unwrap();
	    let txns_de = vec![txn_de];

		// init executor
		let mut executor = FakeExecutor::from_genesis_file();

		if Init::get() == true {
			Init::put(false)
		}else {
			//deseri store_data
			let stored_data :FakeDataStore = Self::load_data_back();
			// load store_data
			executor.set_up_data_store(stored_data);
		}

		// execute block of transcations
		let output = executor.execute_block(txns_de);

		println!("{:?}",output);
		// save store_data on substrate
		Self::store_the_data(&mut executor);

		Ok(())
	}

	#[cfg(feature = "std")]
	pub fn store_the_data(executor: &mut FakeExecutor){
		let mut data_store = executor.get_data_store();
		Self::hash_map_iter_and_seri(&mut data_store);
        //Self::save_data(&mut data_store);
	}

	#[cfg(feature = "std")]
	pub fn save_data(store:&mut FakeDataStore){
		let hashmap = store.get_hash_map();
        println!("start to seri");
		let sered = serde_json::to_vec(&hashmap).unwrap();
		StoreData::put(&sered);
	}

	#[cfg(feature = "std")]
	pub fn hash_map_iter_and_seri(store:&mut FakeDataStore){
		let hashmap = store.get_hash_map();
		let mut store_vec = Vec::new();

		for (a,b) in hashmap.into_iter(){
			let sered_accesspath = serde_json::to_vec(&a.clone()).unwrap();
			store_vec.push((sered_accesspath,b));
		}

		let finalpro = serde_json::to_vec(&store_vec).unwrap();
		Libra_Hash_Map::put(finalpro);
	}

	#[cfg(feature = "std")]
	pub fn load_data_back() -> FakeDataStore{
		println!("start to load data back");
		let data = Libra_Hash_Map::get();
		let mut hashmap = Vechashmap::new();

		let data2 : Vec<(Vec<u8>,Vec<u8>)> =  serde_json::from_slice(&data[..]).unwrap();
		for (a,b) in data2 {
		    let path : AccessPath = serde_json::from_slice(&a[..]).unwrap();
			hashmap.insert(path,b);
		}

		let fake_data_store = FakeDataStore::new(hashmap);

		fake_data_store
	}

	#[cfg(feature = "std")]
	pub fn access_path_test(){
		let new = AccessPath::new(AccountAddress::random(),vec![0u8]);
		let sered = serde_json::to_vec(&new).unwrap();

		let back:AccessPath = serde_json::from_slice(&sered[..]).unwrap();
	}

	#[cfg(feature = "std")]
	pub fn load_data() -> FakeDataStore{
		let data = StoreData::get();
		//let data2 = SimpleDeserializer::deserialize(&data).unwrap();
		let data2 : FakeDataStore =  serde_json::from_slice(&data[..]).unwrap();
		data2
	}

	#[cfg(feature = "std")]
	pub fn move_contratc_generate_signedtx() -> SignedTransaction {
		let mut executor = FakeExecutor::from_genesis_file();
		let sender = AccountData::new(1_000_000, 10);
		executor.add_account_data(&sender);

		let program = String::from(
			"
        module M {
            public foo(u: u64): u64 * u64 * u64 {
                let twice: u64;
                let quadruple: u64;
                twice = 2 * copy(u);
                quadruple = 4 * copy(u);
                return move(u), move(twice), move(quadruple);
            }

            public bar(): u64 {
                return 2;
            }
        }
        ",
		);

		let random_script = compile_program_with_address(sender.address(), &program, vec![]);
		let txn = sender.account().create_signed_txn_impl(*sender.address(), random_script, 10, 100_000, 1);
	    txn
	}


	#[cfg(feature = "std")]
	pub fn create_association_account(){
		//create an create_association_account
		let genesis_account = Account::new_association();
		println!("genesis_account is {:?}",genesis_account);
	}

}
