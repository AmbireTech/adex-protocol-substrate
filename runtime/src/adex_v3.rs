use parity_codec::Encode;
use srml_support::{StorageValue, dispatch::Result};
use {balances, system::{self, ensure_signed}};
use runtime_primitives::traits::Member;
pub trait Trait: balances::Trait {}

extern crate substrate_primitives as primitives;
type AccountId = primitives::H256;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
pub struct Channel {
    creator: AccountId,
    deposit: u128,
    validators: Vec<AccountId>,
    valid_until: u64,
    spec: Vec<u8>,
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn channel_start(origin, channel: Channel) -> Result {
            let sender = ensure_signed(origin)?;

		    //<balances::Module<T>>::decrease_free_balance(&sender, payment)?;
		    Ok(())
        }
		//fn channel_finalize(origin, channel: Commitment<T::AccountId, T::Balance>) -> Result;
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

