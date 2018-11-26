use parity_codec::Encode;
use srml_support::{StorageValue, dispatch::Result};
use {balances, system::{self, ensure_signed}};

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
            // @TODO: move this to impl<T: Trait> Module<T>
            let sender = ensure_signed(origin)?;
            println!("{:?}", channel);

		    //<balances::Module<T>>::decrease_free_balance(&sender, payment)?;
		    Ok(())
        }
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as AdExV3 {
		Payment get(payment) config(): Option<T::Balance>;
        // @TODO Balance should carry multiple tokens
        // @TODO system to clean-up old channels
	}
}

impl<T: Trait> Module<T> {
    //fn channel_finalize(_: T::Origin, channel: Commitment<T::AccountId, T::Balance>) -> Result {
    //    Ok(())
    //}
}

