// --- substrate ---
use sp_runtime::ModuleId;
// --- oracol ---
use crate::*;
use oracol_ethereum_backing::Config;

frame_support::parameter_types! {
	pub const EthereumBackingModuleId: ModuleId = ModuleId(*b"da/ethbk");
	pub const EthereumBackingFeeModuleId: ModuleId = ModuleId(*b"da/ethfe");
	pub const XorLockLimit: Balance = 10_000_000 * COIN;
	pub const OxorLockLimit: Balance = 1000 * COIN;
	pub const AdvancedFee: Balance = 50 * COIN;
	pub const SyncReward: Balance = 1000 * COIN;
}
impl Config for Runtime {
	type ModuleId = EthereumBackingModuleId;
	type FeeModuleId = EthereumBackingFeeModuleId;
	type Event = Event;
	type RedeemAccountId = AccountId;
	type EthereumRelay = EthereumRelay;
	type OnDepositRedeem = Staking;
	type XorCurrency = Xor;
	type OxorCurrency = Oxor;
	type XorLockLimit = XorLockLimit;
	type OxorLockLimit = OxorLockLimit;
	type AdvancedFee = AdvancedFee;
	type SyncReward = SyncReward;
	type EcdsaAuthorities = EthereumRelayAuthorities;
	type WeightInfo = ();
}
