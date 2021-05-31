// --- substrate ---
use sp_runtime::ModuleId;
// --- oracol ---
use crate::*;
use oracol_oldxor_backing::Config;

frame_support::parameter_types! {
	pub const OldxorBackingModuleId: ModuleId = ModuleId(*b"da/oldok");
}
impl Config for Runtime {
	type ModuleId = OldxorBackingModuleId;
	type XorCurrency = Xor;
	type WeightInfo = ();
}
