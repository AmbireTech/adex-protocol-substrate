use primitives::H256;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
pub struct Channel<AccountId, Balance, Moment> {
	pub creator: AccountId,
	pub deposit: Balance,
        pub valid_until: Moment,
	pub validators: Vec<H256>,
	pub spec: Vec<u8>,
}

#[derive(Decode, Encode, PartialEq)]
pub enum ChannelState { Active, Expired }
