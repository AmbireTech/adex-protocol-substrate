use srml_support::{StorageMap, StorageValue, dispatch::Result};
use {balances, system::ensure_signed};

pub mod channel;

use self::channel::{Channel, ChannelState, ChannelId};

pub trait Trait: balances::Trait {}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn channel_start(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
            channel.is_sender_creator(ensure_signed(origin)?)?;
            let id = channel.id();
            assert!(<State<T>>::get(&id) == None, "The channel must be unknown");
            <balances::Module<T>>::decrease_free_balance(&channel.creator, channel.deposit)?;
            <State<T>>::insert(&id, ChannelState::Active as u32);
            Ok(())
        }

        fn channel_withdraw_expired(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
            channel.is_sender_creator(ensure_signed(origin)?)?;
            let id = channel.id();
            assert!(<State<T>>::get(&id) == Some(ChannelState::Active as u32), "The channel must be active");
            <balances::Module<T>>::increase_free_balance_creating(&channel.creator, channel.deposit);
            <State<T>>::insert(id, ChannelState::Expired as u32);
            Ok(())
        }

        fn channel_withdraw(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
            channel.is_sender_creator(ensure_signed(origin)?)?;
            let id = channel.id();
            assert!(<State<T>>::get(&id) == Some(ChannelState::Active as u32), "The channel must be active");
            // @TODO: check state
            // @TODO check balance leaf and etc.
            Ok(())
        }
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as AdExOUTPACE {
        Dummy get(dummy) config(): u32; // needed for GenesisConfig generation
        pub State get(state): map ChannelId => Option<u32>;
        pub Withdrawn get(withdrawn): map ChannelId => Option<T::Balance>;
        pub WithdrawnPerUser get(withdrawn_per_user): map (ChannelId, T::AccountId) => Option<T::Balance>;
    }
}

impl<T: Trait> Module<T> {}

