//! The Substrate Node Template runtime. This can be compiled with `#[no_std]`, ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit="256"]

extern crate sr_std as rstd;
extern crate sr_io as runtime_io;
#[macro_use]
extern crate substrate_client as client;
#[macro_use]
extern crate srml_support;
#[macro_use]
extern crate sr_primitives as runtime_primitives;
#[cfg(feature = "std")]
#[macro_use]
extern crate serde_derive;
extern crate substrate_primitives as primitives;
extern crate parity_codec;
#[macro_use]
extern crate parity_codec_derive;
#[macro_use]
extern crate sr_version as version;
extern crate srml_system as system;
extern crate srml_executive as executive;
extern crate srml_consensus as consensus;
extern crate srml_timestamp as timestamp;
extern crate srml_balances as balances;
extern crate srml_upgrade_key as upgrade_key;

#[cfg(feature = "std")]
use parity_codec::{Encode, Decode};
use rstd::prelude::*;
#[cfg(feature = "std")]
use primitives::bytes;
use primitives::AuthorityId;
use primitives::OpaqueMetadata;
use runtime_primitives::{ApplyResult, transaction_validity::TransactionValidity,
	Ed25519Signature, generic, traits::{self, BlakeTwo256, Block as BlockT}
};
#[cfg(feature = "std")]
use runtime_primitives::traits::ApiRef;
use client::{block_builder::api::runtime::*, runtime_api::{runtime::*, id::*}};
#[cfg(feature = "std")]
use client::runtime_api::ApiExt;
use version::RuntimeVersion;
#[cfg(feature = "std")]
use version::NativeVersion;

// A few exports that help ease life for downstream crates.
#[cfg(any(feature = "std", test))]
pub use runtime_primitives::BuildStorage;
pub use consensus::Call as ConsensusCall;
pub use timestamp::Call as TimestampCall;
pub use balances::Call as BalancesCall;
pub use runtime_primitives::{Permill, Perbill};
pub use timestamp::BlockPeriod;
pub use srml_support::{StorageValue, RuntimeMetadata};

mod adex_v3;

/// Alias to Ed25519 pubkey that identifies an account on the chain.
pub type AccountId = primitives::H256;

/// A hash of some data used by the chain.
pub type Hash = primitives::H256;

/// Index of a block number in the chain.
pub type BlockNumber = u64;

/// Index of an account's extrinsic in the chain.
pub type Nonce = u64;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core datastructures.
pub mod opaque {
	use super::*;

	/// Opaque, encoded, unchecked extrinsic.
	#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
	pub struct UncheckedExtrinsic(#[cfg_attr(feature = "std", serde(with="bytes"))] pub Vec<u8>);
	impl traits::Extrinsic for UncheckedExtrinsic {
		fn is_signed(&self) -> Option<bool> {
			None
		}
	}
	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256, generic::DigestItem<Hash, AuthorityId>>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
}

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: ver_str!("adex-protocol-substrate"),
	impl_name: ver_str!("adex-protocol-substrate"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 0,
	apis: apis_vec!([
		(BLOCK_BUILDER, 1),
		(TAGGED_TRANSACTION_QUEUE, 1),
		(METADATA, 1)
	]),
};

/// The version infromation used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

impl system::Trait for Runtime {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Nonce;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header digest type.
	type Digest = generic::Digest<Log>;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
	/// The ubiquitous event type.
	type Event = Event;
	/// The ubiquitous log type.
	type Log = Log;
	/// The ubiquitous origin type.
	type Origin = Origin;
}

impl consensus::Trait for Runtime {
	/// The position in the block's extrinsics that the note-offline inherent must be placed.
	const NOTE_OFFLINE_POSITION: u32 = 1;
	/// The identifier we use to refer to authorities.
	type SessionKey = AuthorityId;
	/// No action in case an authority was determined to be offline.
	type OnOfflineValidator = ();
	/// The ubiquitous log type.
	type Log = Log;
}

impl timestamp::Trait for Runtime {
	/// The position in the block's extrinsics that the timestamp-set inherent must be placed.
	const TIMESTAMP_SET_POSITION: u32 = 0;
	/// A timestamp: seconds since the unix epoch.
	type Moment = u64;
}

impl balances::Trait for Runtime {
	/// The type for recording an account's balance.
	type Balance = u128;
	/// The type for recording indexing into the account enumeration. If this ever overflows, there
	/// will be problems!
	type AccountIndex = u32;
	/// What to do if an account's free balance gets zeroed.
	type OnFreeBalanceZero = ();
	/// Restrict whether an account can transfer funds. We don't place any further restrictions.
	type EnsureAccountLiquid = ();
	/// The uniquitous event type.
	type Event = Event;
}

impl upgrade_key::Trait for Runtime {
	/// The uniquitous event type.
	type Event = Event;
}

impl adex_v3::Trait for Runtime {}

construct_runtime!(
	pub enum Runtime with Log(InternalLog: DigestItem<Hash, AuthorityId>) where
		Block = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system::{default, Log(ChangesTrieRoot)},
		Timestamp: timestamp::{Module, Call, Storage, Config<T>, Inherent},
		Consensus: consensus::{Module, Call, Storage, Config<T>, Log(AuthoritiesChange), Inherent},
		Balances: balances,
                AdEx: adex_v3::{Module, Call, Storage, Config<T>},
		UpgradeKey: upgrade_key,
	}
);

/// The type used as a helper for interpreting the sender of transactions. 
type Context = balances::ChainContext<Runtime>;
/// The address format for describing accounts.
type Address = balances::Address<Runtime>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedMortalExtrinsic<Address, Nonce, Call, Ed25519Signature>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Nonce, Call>;
/// Executive: handles dispatch to the various modules.
pub type Executive = executive::Executive<Runtime, Block, Context, Balances, AllModules>;

#[cfg(feature = "std")]
use opaque::Block as GBlock;

#[cfg(feature = "std")]
pub struct ClientWithApi {
	call: ::std::ptr::NonNull<client::runtime_api::CallApiAt<GBlock>>,
	commit_on_success: ::std::cell::RefCell<bool>,
	initialised_block: ::std::cell::RefCell<Option<GBlockId>>,
	changes: ::std::cell::RefCell<client::runtime_api::OverlayedChanges>,
}

#[cfg(feature = "std")]
unsafe impl Send for ClientWithApi {}
#[cfg(feature = "std")]
unsafe impl Sync for ClientWithApi {}

#[cfg(feature = "std")]
impl ApiExt for ClientWithApi {
	fn map_api_result<F: FnOnce(&Self) -> Result<R, E>, R, E>(&self, map_call: F) -> Result<R, E> {
		*self.commit_on_success.borrow_mut() = false;
		let res = map_call(self);
		*self.commit_on_success.borrow_mut() = true;

		self.commit_on_ok(&res);

		res
	}
}

#[cfg(feature = "std")]
impl client::runtime_api::ConstructRuntimeApi<GBlock> for ClientWithApi {
	fn construct_runtime_api<'a, T: client::runtime_api::CallApiAt<GBlock>>(call: &'a T) -> ApiRef<'a, Self> {
		ClientWithApi {
			call: unsafe {
				::std::ptr::NonNull::new_unchecked(
					::std::mem::transmute(
						call as &client::runtime_api::CallApiAt<GBlock>
					)
				)
			},
			commit_on_success: true.into(),
			initialised_block: None.into(),
			changes: Default::default(),
		}.into()
	}
}

#[cfg(feature = "std")]
impl ClientWithApi {
	fn call_api_at<A: Encode, R: Decode>(
		&self,
		at: &GBlockId,
		function: &'static str,
		args: &A
	) -> client::error::Result<R> {
		let res = unsafe {
			self.call.as_ref().call_api_at(
				at,
				function,
				args.encode(),
				&mut *self.changes.borrow_mut(),
				&mut *self.initialised_block.borrow_mut()
			).and_then(|r|
				R::decode(&mut &r[..])
					.ok_or_else(||
						client::error::ErrorKind::CallResultDecode(function).into()
					)
			)
		};

		self.commit_on_ok(&res);
		res
	}

	fn commit_on_ok<R, E>(&self, res: &Result<R, E>) {
		if *self.commit_on_success.borrow() {
			if res.is_err() {
				self.changes.borrow_mut().discard_prospective();
			} else {
				self.changes.borrow_mut().commit_prospective();
			}
		}
	}
}

#[cfg(feature = "std")]
type GBlockId = generic::BlockId<GBlock>;

#[cfg(feature = "std")]
impl client::runtime_api::Core<GBlock> for ClientWithApi {
	fn version(&self, at: &GBlockId) -> Result<RuntimeVersion, client::error::Error> {
		self.call_api_at(at, "version", &())
	}

	fn authorities(&self, at: &GBlockId) -> Result<Vec<AuthorityId>, client::error::Error> {
		self.call_api_at(at, "authorities", &())
	}

	fn execute_block(&self, at: &GBlockId, block: &GBlock) -> Result<(), client::error::Error> {
		self.call_api_at(at, "execute_block", block)
	}

	fn initialise_block(&self, at: &GBlockId, header: &<GBlock as BlockT>::Header) -> Result<(), client::error::Error> {
		self.call_api_at(at, "initialise_block", header)
	}
}

#[cfg(feature = "std")]
impl client::block_builder::api::BlockBuilder<GBlock> for ClientWithApi {
	fn apply_extrinsic(&self, at: &GBlockId, extrinsic: &<GBlock as BlockT>::Extrinsic) -> Result<ApplyResult, client::error::Error> {
		self.call_api_at(at, "apply_extrinsic", extrinsic)
	}

	fn finalise_block(&self, at: &GBlockId) -> Result<<GBlock as BlockT>::Header, client::error::Error> {
		self.call_api_at(at, "finalise_block", &())
	}

	fn inherent_extrinsics<Inherent: Decode + Encode, Unchecked: Decode + Encode>(
		&self, at: &GBlockId, inherent: &Inherent
	) -> Result<Vec<Unchecked>, client::error::Error> {
		self.call_api_at(at, "inherent_extrinsics", inherent)
	}

	fn check_inherents<Inherent: Decode + Encode, Error: Decode + Encode>(&self, at: &GBlockId, block: &GBlock, inherent: &Inherent) -> Result<Result<(), Error>, client::error::Error> {
		self.call_api_at(at, "check_inherents", &(block, inherent))
	}

	fn random_seed(&self, at: &GBlockId) -> Result<<GBlock as BlockT>::Hash, client::error::Error> {
		self.call_api_at(at, "random_seed", &())
	}
}

#[cfg(feature = "std")]
impl client::runtime_api::TaggedTransactionQueue<GBlock> for ClientWithApi {
	fn validate_transaction(
		&self,
		at: &GBlockId,
		utx: &<GBlock as BlockT>::Extrinsic
	) -> Result<TransactionValidity, client::error::Error> {
		self.call_api_at(at, "validate_transaction", utx)
	}
}

#[cfg(feature = "std")]
impl client::runtime_api::Metadata<GBlock> for ClientWithApi {
	fn metadata(&self, at: &GBlockId) -> Result<OpaqueMetadata, client::error::Error> {
		self.call_api_at(at, "metadata", &())
	}
}

// Implement our runtime API endpoints. This is just a bunch of proxying.
impl_runtime_apis! {
	impl Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn authorities() -> Vec<AuthorityId> {
			Consensus::authorities()
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialise_block(header: <Block as BlockT>::Header) {
			Executive::initialise_block(&header)
		}
	}

	impl Metadata for Runtime {
		fn metadata() -> OpaqueMetadata {
			Runtime::metadata().into()
		}
	}

	impl BlockBuilder<Block, InherentData, UncheckedExtrinsic, InherentData, InherentError> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalise_block() -> <Block as BlockT>::Header {
			Executive::finalise_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<UncheckedExtrinsic> {
			data.create_inherent_extrinsics()
		}

		fn check_inherents(block: Block, data: InherentData) -> Result<(), InherentError> {
			data.check_inherents(block)
		}

		fn random_seed() -> <Block as BlockT>::Hash {
			System::random_seed()
		}
	}

	impl TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
			Executive::validate_transaction(tx)
		}
	}
}
