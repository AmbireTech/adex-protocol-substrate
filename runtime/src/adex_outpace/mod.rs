use srml_support::{StorageValue, dispatch::Result};
use {balances, system::ensure_signed};

pub mod channel;

use self::channel::Channel;

pub trait Trait: balances::Trait {}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn channel_start(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
            channel.is_sender_creator(ensure_signed(origin)?)?;
            <balances::Module<T>>::decrease_free_balance(&channel.creator, channel.deposit)?;
            // @TODO set state
            Ok(())
        }

        fn channel_withdraw_expired(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
            channel.is_sender_creator(ensure_signed(origin)?)?;
            // @TODO check state
            <balances::Module<T>>::increase_free_balance_creating(&channel.creator, channel.deposit);
            Ok(())
        }

        fn channel_withdraw(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
            channel.is_sender_creator(ensure_signed(origin)?)?;
            // @TODO check state
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
        pub Withdrawn get(withdrawn): map T::Hash => Option<T::Balance>;
        pub WithdrawnPerUser get(withdrawn_per_user): map (T::Hash, T::AccountId) => Option<T::Balance>;
    }
}

impl<T: Trait> Module<T> {}

