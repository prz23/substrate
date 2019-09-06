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
	ConsensusEngineId,decl_event,
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
use types::transaction::SignedTransaction;
#[cfg(feature = "std")]
use executor::*;
#[cfg(feature = "std")]
use config::config::VMConfig;
#[cfg(feature = "std")]
use vm_runtime::MoveVM;


pub trait Trait: timestamp::Trait {
	/// The identifier type for an authority.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as Libra {
		/// The last timestamp.
		LastTimestamp get(last) build(|_| 0.into()): T::Moment;

	}
}


decl_event!(
	pub enum Event<T> where <T as system::Trait>::AccountId {
		OfflineWarning(AccountId, u32),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin { }
}

impl<T: Trait> Module<T> {

	fn initialize_authorities() {
		let vm = MoveVM::new(&VMConfig::default());
	}
}

