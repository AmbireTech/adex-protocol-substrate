use srml_support::{StorageValue, dispatch::Result};
use {balances, system::ensure_signed};

pub trait Trait: balances::Trait {}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
pub struct Channel<AccountId, Balance> {
    creator: AccountId,
    deposit: Balance,
    // @TODO should valid_until be some sort of substrate-specific lifetime value?
    valid_until: u64,
    validators: Vec<AccountId>,
    spec: Vec<u8>,
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn channel_start(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
            let sender = ensure_signed(origin)?;
            <balances::Module<T>>::decrease_free_balance(&sender, channel.deposit)?;
            // @TODO set state
		    Ok(())
        }

        fn channel_withdraw_expired(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
            let sender = ensure_signed(origin)?;
            // @TODO check state
            <balances::Module<T>>::increase_free_balance_creating(&sender, channel.deposit);
            Ok(())
        }

        fn channel_withdraw(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
            // @TODO: check state
            // @TODO check balance leaf and etc.
            Ok(())
        }
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as AdExV3 {
            // payment is a dummy to make decl_storage! generate a GenesisConfig
            pub Payment get(payment) config(): Option<T::Balance>;
            pub State get(state): map T::Hash => Option<u32>;
            pub Withdrawn get(withdrawn): map T::Hash => Option<T::Balance>;
            pub WithdrawnPerUser get(withdrawn_per_user): map (T::Hash, T::AccountId) => Option<T::Balance>;
	}
}

impl<T: Trait> Module<T> {}

