use srml_support::{StorageMap, dispatch::Result};
use {balances, timestamp, system::ensure_signed};
use runtime_primitives::traits::Hash;

#[cfg(test)]
extern crate sr_io as runtime_io;
use runtime_io::ed25519_verify;

extern crate sr_std as rstd;
use rstd::prelude::*;

pub mod channel;

use self::channel::{Channel, ChannelState};

pub trait Trait: balances::Trait + timestamp::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode)]
struct Two<A, B> (A, B);
impl<T> Two<T, T> where T: AsRef<[u8]> {
    fn combine_sorted(a: T, b: T) -> Self {
            if a.as_ref() < b.as_ref() {
                    Two(a, b)
            } else {
                    Two(b, a)
            }
    }
}

type Signature = primitives::H512;

// Implements OUTPACE: https://github.com/AdExNetwork/adex-protocol/blob/master/OUTPACE.md
// Off-chain Unidirectional Trustless PAyment ChannEls

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		fn channel_open(origin, channel: Channel<T::AccountId, T::Balance, T::Moment>) -> Result {
			ensure!(ensure_signed(origin)? == channel.creator, "the sender must be channel.creator");
			ensure!(channel.is_valid(), "the channel must be valid");

			let channel_hash = T::Hashing::hash_of(&channel);
			ensure!(
				!<State<T>>::exists(&channel_hash),
				"channel must not already exist"
			);
			<State<T>>::insert(&channel_hash, ChannelState::Active);
			<balances::Module<T>>::decrease_free_balance(&channel.creator, channel.deposit)?;
			Self::deposit_event(RawEvent::ChannelOpen(channel_hash));
			Ok(())
		}

		fn channel_withdraw_expired(origin, channel: Channel<T::AccountId, T::Balance, T::Moment>) -> Result {
			ensure!(
				ensure_signed(origin)? == channel.creator,
				"the sender must be channel.creator"
			);
			let channel_hash = T::Hashing::hash_of(&channel);
			ensure!(
				<State<T>>::get(&channel_hash) == Some(ChannelState::Active),
				"channel must be active"
			);
			ensure!(
				<timestamp::Module<T>>::get() > channel.valid_until,
				"channel must be expired"
			);

			<State<T>>::insert(channel_hash, ChannelState::Expired);

			let to_withdraw = channel.deposit - Self::withdrawn(&channel_hash);
			<balances::Module<T>>::increase_free_balance_creating(&channel.creator, to_withdraw);

			Self::deposit_event(RawEvent::ChannelWithdrawExpired(channel_hash, to_withdraw));
			Ok(())
		}

		fn channel_withdraw(
			origin,
			channel: Channel<T::AccountId, T::Balance, T::Moment>,
			state_root: T::Hash,
			signatures: Vec<Signature>,
			proof: Vec<T::Hash>,
			amount_in_tree: T::Balance
		) -> Result {
			let sender = ensure_signed(origin)?;
			let channel_hash = T::Hashing::hash_of(&channel);

			// Check if the channel is in an Active state and not expired
			ensure!(
				<State<T>>::get(&channel_hash) == Some(ChannelState::Active),
				"channel must be active"
			);
			ensure!(
				<timestamp::Module<T>>::get() <= channel.valid_until,
				"channel must not be expired"
			);

			// Check if the state is signed by a supermajority of validators
			ensure!(
				signatures.len() == channel.validators.len(),
				"signatures must be as many as validators"
			);
			let to_sign = T::Hashing::hash_of(&Two(channel_hash, state_root));
			let valid_sigs = signatures.iter()
				.zip(channel.validators.iter())
				.filter(|(sig, validator)| {
					ed25519_verify(&sig.to_fixed_bytes(), to_sign.as_ref(), validator.to_fixed_bytes())
				})
				.count();
			ensure!(
				valid_sigs*3 >= channel.validators.len()*2,
				"state must be signed by a validator supermajority"
			);

			// Check the merkle inclusion proof for the balance leaf
			let balance_leaf = T::Hashing::hash_of(&Two(sender.clone(), amount_in_tree));
			let is_contained = state_root == proof.iter()
				.fold(balance_leaf, |a, b| {
					T::Hashing::hash_of(&Two::combine_sorted(a.clone(), b.clone()))
				});
			ensure!(is_contained, "balance leaf not found");

			// Calculate how much the user has left to withdraw
			let withdrawn_key = (channel_hash.clone(), sender.clone());
			let withdrawn_so_far = Self::withdrawn_per_user(&withdrawn_key);
			ensure!(
				amount_in_tree > withdrawn_so_far,
				"amount_in_tree should be larger than withdrawn_so_far"
			);
			let to_withdraw = amount_in_tree - withdrawn_so_far;
			<WithdrawnPerUser<T>>::insert(&withdrawn_key, amount_in_tree);

			// Ensure it's not possible to withdraw more than the channel balance
			let withdrawn_total = Self::withdrawn(&channel_hash) + to_withdraw;
			ensure!(
				withdrawn_total <= channel.deposit,
				"total withdrawn must not exceed channel deposit"
			);
			<Withdrawn<T>>::insert(&channel_hash, withdrawn_total);

			<balances::Module<T>>::increase_free_balance_creating(&sender, to_withdraw);

			Self::deposit_event(RawEvent::ChannelWithdraw(sender, channel_hash, to_withdraw));
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

decl_event!(
        pub enum Event<T> where
		<T as system::Trait>::Hash,
		<T as system::Trait>::AccountId,
		<T as balances::Trait>::Balance
	{
		ChannelOpen(Hash),
		ChannelWithdrawExpired(Hash, Balance),
		ChannelWithdraw(AccountId, Hash, Balance),
	}
);

impl<T: Trait> Module<T> {}

