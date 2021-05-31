// --- oracol ---
pub use oracol_relayer_game::Instance0 as EthereumRelayerGameInstance;

// --- substrate ---
use frame_support::traits::LockIdentifier;
// --- oracol ---
use crate::*;
use oracol_relayer_game::Config;

frame_support::parameter_types! {
	pub const EthereumRelayerGameLockId: LockIdentifier = *b"ethrgame";
}
impl Config<EthereumRelayerGameInstance> for Runtime {
	type XorCurrency = Xor;
	type LockId = EthereumRelayerGameLockId;
	type XorSlash = Treasury;
	type RelayerGameAdjustor = relay::EthereumRelayerGameAdjustor;
	type RelayableChain = EthereumRelay;
	type WeightInfo = ();
}
