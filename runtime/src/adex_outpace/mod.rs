use srml_support::{StorageMap, dispatch::Result};
use {balances, system::ensure_signed};
use runtime_primitives::traits::Hash;

pub mod channel;

use self::channel::{Channel, ChannelState};

pub trait Trait: balances::Trait {}

#[derive(Encode, Decode)]
struct Both<A, B> { a: A, b: B }

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn channel_start(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
			ensure!(ensure_signed(origin)? == channel.creator, "the sender must be channel.creator");
			let channel_hash = T::Hashing::hash_of(&channel);
			ensure!(!<State<T>>::exists(&channel_hash), "channel already exists");
			// @TODO: is_valid
			<State<T>>::insert(&channel_hash, ChannelState::Active);
			<balances::Module<T>>::decrease_free_balance(&channel.creator, channel.deposit)?;
			Ok(())
		}

		fn channel_withdraw_expired(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
			ensure!(ensure_signed(origin)? == channel.creator, "the sender must be channel.creator");
			let channel_hash = T::Hashing::hash_of(&channel);
			ensure!(<State<T>>::get(&channel_hash) == Some(ChannelState::Active), "channel must be active");
			// @TODO: check if expired
			// @TODO: only withdraw remaining balance
			<State<T>>::insert(channel_hash, ChannelState::Expired);
			<balances::Module<T>>::increase_free_balance_creating(&channel.creator, channel.deposit);
			Ok(())
		}

		fn channel_withdraw(
			origin,
			channel: Channel<T::AccountId, T::Balance>,
			state_root: T::Hash,
			signatures: Vec<u8>,
			proof: Vec<T::Hash>,
			amountInTree: u64
		) -> Result {
			let sender = ensure_signed(origin)?;
			let channel_hash = T::Hashing::hash_of(&channel);
			ensure!(<State<T>>::get(&channel_hash) == Some(ChannelState::Active), "channel must be active");
			// @TODO: check if NOT expired
			let to_sign = T::Hashing::hash_of(&Both{ a: channel_hash, b: state_root });
			// ensure!(channel.is_signed_by_supermajority(to_sign, signatures), "state must be signed");
			let balance_leaf = T::Hashing::hash_of(&Both{ a: sender, b: amountInTree });
			let is_contained = state_root == proof.iter().fold(balance_leaf, |a, b| {
				// https://github.com/paritytech/parity-common/blob/master/fixed-hash/src/hash.rs#L101
				T::Hashing::hash_of(if a.as_ref() < b.as_ref() { &a } else { &b })
			});
			ensure!(is_contained, "balance leaf not found");
			// @TODO; withdraw the actual balance, check Withdrawn, WithdrawnPerUser
			Ok(())
		}
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as AdExOUTPACE {
		pub State get(state): map T::Hash => Option<ChannelState>;
		pub Withdrawn get(withdrawn): map T::Hash => T::Balance;
		pub WithdrawnPerUser get(withdrawn_per_user): map (T::Hash, T::AccountId) => T::Balance;
	}
}

impl<T: Trait> Module<T> {}

