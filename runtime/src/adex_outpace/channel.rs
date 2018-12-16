use primitives::H256;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
pub struct Channel<AccountId, Balance> {
	pub creator: AccountId,
	pub deposit: Balance,
	// @TODO should valid_until be some sort of substrate-specific lifetime value?
	pub valid_until: u64,
	pub validators: Vec<H256>,
	pub spec: Vec<u8>,
}

#[derive(Decode, Encode, PartialEq)]
pub enum ChannelState { Active, Expired }
