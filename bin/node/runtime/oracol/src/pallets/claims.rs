// --- substrate ---
use sp_runtime::ModuleId;
// --- oracol ---
use crate::*;
use oracol_claims::Config;

frame_support::parameter_types! {
	pub const ClaimsModuleId: ModuleId = ModuleId(*b"da/claim");
	pub Prefix: &'static [u8] = b"Pay PXORs to the Oracol account:";
}
impl Config for Runtime {
	type Event = Event;
	type ModuleId = ClaimsModuleId;
	type Prefix = Prefix;
	type XorCurrency = Xor;
	type MoveClaimOrigin = EnsureRootOrMoreThanHalfCouncil;
}
