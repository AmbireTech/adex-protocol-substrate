use srml_support::{StorageMap, dispatch::Result};
use {balances, system::ensure_signed};
use runtime_primitives::traits::Hash;
use primitives::ed25519;

pub mod channel;

use self::channel::{Channel, ChannelState};

pub trait Trait: balances::Trait {}

#[derive(Encode, Decode)]
struct Both<A, B> { a: A, b: B }

type Signature = ed25519::Signature;

// Implements OUTPACE: https://github.com/AdExNetwork/adex-protocol/blob/master/OUTPACE.md
// Off-chain Unidirectional Trustless PAyment ChannEls

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn channel_start(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
			ensure!(
				ensure_signed(origin)? == channel.creator,
				"the sender must be channel.creator"
			);
			let channel_hash = T::Hashing::hash_of(&channel);
			ensure!(
				!<State<T>>::exists(&channel_hash),
				"channel must not already exist"
			);
			<State<T>>::insert(&channel_hash, ChannelState::Active);
			<balances::Module<T>>::decrease_free_balance(&channel.creator, channel.deposit)?;
			Ok(())
		}

		fn channel_withdraw_expired(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
			ensure!(
				ensure_signed(origin)? == channel.creator,
				"the sender must be channel.creator"
			);
			let channel_hash = T::Hashing::hash_of(&channel);
			ensure!(
				<State<T>>::get(&channel_hash) == Some(ChannelState::Active),
				"channel must be active"
			);

			<State<T>>::insert(channel_hash, ChannelState::Expired);

			let to_withdraw =channel.deposit - Self::withdrawn(&channel_hash);
			<balances::Module<T>>::increase_free_balance_creating(&channel.creator, to_withdraw);

			Ok(())
		}

		fn channel_withdraw(
			origin,
			channel: Channel<T::AccountId, T::Balance>,
			state_root: T::Hash,
			signatures: Vec<Signature>,
			proof: Vec<T::Hash>,
			amount_in_tree: T::Balance
		) -> Result {
			let sender = ensure_signed(origin)?;
			let channel_hash = T::Hashing::hash_of(&channel);
			ensure!(
				<State<T>>::get(&channel_hash) == Some(ChannelState::Active),
				"channel must be active"
			);
			// @TODO: check if NOT expired

			// Check if the state is signed by a supermajority of validators
			ensure!(
				signatures.len() == channel.validators.len(),
				"signatures must be as many as validators"
			);
			let to_sign = T::Hashing::hash_of(&Both{ a: channel_hash, b: state_root });
			let valid_sigs = signatures.iter()
				.zip(channel.validators.iter())
				.filter(|(sig, validator)| {
					let public = ed25519::Public::from_raw(validator.to_fixed_bytes());
					ed25519::verify_strong(sig, to_sign.as_ref(), public)
				})
				.count();
			ensure!(
				valid_sigs*3 >= channel.validators.len()*2,
				"state must be signed by a validator supermajority"
			);

			// Check the merkle inclusion proof for the balance leaf
			let balance_leaf = T::Hashing::hash_of(&Both{ a: sender.clone(), b: amount_in_tree });
			let is_contained = state_root == proof.iter()
				.fold(balance_leaf, |a, b| {
					T::Hashing::hash_of(if a.as_ref() < b.as_ref() { &a } else { &b })
				});
			ensure!(
				is_contained,
				"balance leaf not found"
			);

			// Calculate how much the user has left to withdraw
			let withdrawn_so_far = Self::withdrawn_per_user((channel_hash.clone(), sender.clone()));
			ensure!(
				amount_in_tree > withdrawn_so_far,
				"amount_in_tree should be larger"
			);
			let to_withdraw = amount_in_tree - withdrawn_so_far;

			// Ensure it's not possible to withdraw more than the channel balance
			let withdrawn_total = Self::withdrawn(&channel_hash) + to_withdraw;
			ensure!(
				withdrawn_total <= channel.deposit,
				"total withdrawn must not exceed channel deposit"
			);

			<balances::Module<T>>::increase_free_balance_creating(&sender, to_withdraw);

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

