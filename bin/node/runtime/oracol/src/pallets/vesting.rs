// --- substrate ---
use sp_runtime::traits::ConvertInto;
// --- oracol ---
use crate::*;
use oracol_vesting::Config;

frame_support::parameter_types! {
	pub const MinVestedTransfer: Balance = 100 * MILLI;
}
impl Config for Runtime {
	type Event = Event;
	type Currency = Xor;
	type BlockNumberToBalance = ConvertInto;
	type MinVestedTransfer = MinVestedTransfer;
	type WeightInfo = ();
}
