use scale_info::TypeInfo;
use codec::{Decode, Encode, MaxEncodedLen};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use frame_support::sp_runtime::{RuntimeDebug};

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
	Eq, PartialEq, Clone, Copy, Encode, Decode, Default, RuntimeDebug, MaxEncodedLen, TypeInfo,
)]
pub struct FeelessInfo<BlockNumber> {
	pub last_user_session: BlockNumber,
    pub user_calls: u32,
}