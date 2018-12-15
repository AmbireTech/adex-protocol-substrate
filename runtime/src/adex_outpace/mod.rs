use srml_support::{StorageMap, StorageValue, dispatch::Result};
use {balances, system::ensure_signed};
use runtime_primitives::traits::Hash;

pub mod channel;

use self::channel::{Channel, ChannelState};

pub trait Trait: balances::Trait {}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn channel_start(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
            // @TODO: should this be an assert! ?
            channel.is_sender_creator(ensure_signed(origin)?)?;
            let channel_hash = T::Hashing::hash_of(&channel);
            // @TODO: can we use exists() here?
            // @TODO: is_valid
            assert!(<State<T>>::get(&channel_hash) == None, "The channel must be unknown");
            <balances::Module<T>>::decrease_free_balance(&channel.creator, channel.deposit)?;
            <State<T>>::insert(&channel_hash, ChannelState::Active as u32);
            Ok(())
        }

        fn channel_withdraw_expired(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
            channel.is_sender_creator(ensure_signed(origin)?)?;
            let channel_hash = T::Hashing::hash_of(&channel);
            assert!(<State<T>>::get(&channel_hash) == Some(ChannelState::Active as u32), "The channel must be active");
            // @TODO: check if expired
            // @TODO: only withdraw remaining balance
            <balances::Module<T>>::increase_free_balance_creating(&channel.creator, channel.deposit);
            <State<T>>::insert(channel_hash, ChannelState::Expired as u32);
            Ok(())
        }

        fn channel_withdraw(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
            channel.is_sender_creator(ensure_signed(origin)?)?;
            let channel_hash = T::Hashing::hash_of(&channel);
            assert!(<State<T>>::get(&channel_hash) == Some(ChannelState::Active as u32), "The channel must be active");
            // @TODO: check state
            // @TODO check balance leaf and etc.
            Ok(())
        }
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as AdExOUTPACE {
        Dummy get(dummy) config(): u32; // needed for GenesisConfig generation
        pub State get(state): map T::Hash => Option<u32>;
        pub Withdrawn get(withdrawn): map T::Hash => T::Balance;
        pub WithdrawnPerUser get(withdrawn_per_user): map (T::Hash, T::AccountId) => T::Balance;
    }
}

impl<T: Trait> Module<T> {}

