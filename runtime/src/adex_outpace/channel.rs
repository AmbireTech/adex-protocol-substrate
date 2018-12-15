use srml_support::dispatch::Result;

pub type ChannelId = Vec<u8>;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
pub struct Channel<AccountId, Balance> {
    pub creator: AccountId,
    pub deposit: Balance,
    // @TODO should valid_until be some sort of substrate-specific lifetime value?
    pub valid_until: u64,
    pub validators: Vec<AccountId>,
    pub spec: Vec<u8>,
}
impl<AccountId, Balance> Channel<AccountId, Balance>
    where AccountId: PartialEq
{
    pub fn id(&self) -> ChannelId {
        vec![0,0,0,0]
    }
    pub fn is_sender_creator(&self, sender: AccountId) -> Result {
        match sender == self.creator {
            true => Ok(()),
            false => Err("not the channel creator"),
        }
    }
}

pub enum ChannelState { Active, Expired }
