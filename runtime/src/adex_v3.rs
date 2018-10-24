use parity_codec::Encode;
use srml_support::{StorageValue, dispatch::Result};
use {balances, system::{self, ensure_signed}};

pub trait Trait: balances::Trait {}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
pub struct Bid<AccountId> {
    advertiser: AccountId,
    //total_reward: Balance,
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn commitment_start(origin, bid: Bid<T::AccountId>) -> Result;
		//fn commitment_finalize(origin, commitment: Commitment<T::AccountId, T::Balance>) -> Result;
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as AdExV3 {
		Payment get(payment) config(): Option<T::Balance>;
        // @TODO Balance should carry multiple tokens
        // @TODO system to clean-up old commitments
	}
}

impl<T: Trait> Module<T> {
	fn commitment_start(origin: T::Origin, bid: Bid<T::AccountId>) -> Result {
		let sender = ensure_signed(origin)?;
	
		//<balances::Module<T>>::decrease_free_balance(&sender, payment)?;
		Ok(())
	}

    //fn commitment_finalize(_: T::Origin, commitment: Commitment<T::AccountId, T::Balance>) -> Result {
    //    Ok(())
    //}
}

