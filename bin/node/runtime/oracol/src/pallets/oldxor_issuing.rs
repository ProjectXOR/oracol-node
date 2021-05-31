// --- substrate ---
use sp_runtime::ModuleId;
// --- oracol ---
use crate::*;
use oracol_oldxor_issuing::Config;

frame_support::parameter_types! {
	pub const OldxorIssuingModuleId: ModuleId = ModuleId(*b"da/crais");
}
impl Config for Runtime {
	type Event = Event;
	type ModuleId = OldxorIssuingModuleId;
	type XorCurrency = Xor;
	type WeightInfo = ();
}
