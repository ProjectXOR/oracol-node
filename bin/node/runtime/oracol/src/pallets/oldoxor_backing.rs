// --- substrate ---
use sp_runtime::ModuleId;
// --- oracol ---
use crate::*;
use oracol_oldoxor_backing::Config;

frame_support::parameter_types! {
	pub const OldoxorBackingModuleId: ModuleId = ModuleId(*b"da/trobk");
}
impl Config for Runtime {
	type ModuleId = OldoxorBackingModuleId;
	type XorCurrency = Xor;
	type OxorCurrency = Oxor;
	type WeightInfo = ();
}
