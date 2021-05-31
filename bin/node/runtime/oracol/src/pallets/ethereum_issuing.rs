// --- substrate ---
use sp_runtime::ModuleId;
// --- oracol ---
use crate::*;
use oracol_ethereum_issuing::Config;

frame_support::parameter_types! {
	pub const EthereumIssuingModuleId: ModuleId = ModuleId(*b"da/ethis");
}

impl Config for Runtime {
	type ModuleId = EthereumIssuingModuleId;
	type Event = Event;
	type EthereumRelay = EthereumRelay;
	type XorCurrency = Xor;
	type EcdsaAuthorities = EthereumRelayAuthorities;
	type WeightInfo = ();
}
