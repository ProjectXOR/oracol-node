// --- oracol ---
pub use oracol_relay_authorities::Instance0 as EthereumRelayAuthoritiesInstance;

// --- substrate ---
use frame_support::traits::LockIdentifier;
use sp_runtime::Perbill;
// --- oracol ---
use crate::*;
use oracol_relay_authorities::Config;
use oracol_relay_primitives::relay_authorities::OpCode;

frame_support::parameter_types! {
	pub const EthereumRelayAuthoritiesLockId: LockIdentifier = *b"ethrauth";
	pub const EthereumRelayAuthoritiesTermDuration: BlockNumber = 3 * MINUTES;
	pub const MaxCandidates: usize = 7;
	pub const OpCodes: (OpCode, OpCode) = (
		[71, 159, 189, 249],
		[180, 188, 244, 151]
	);
	pub const SignThreshold: Perbill = Perbill::from_percent(60);
	pub const SubmitDuration: BlockNumber = 30;
}
impl Config<EthereumRelayAuthoritiesInstance> for Runtime {
	type Event = Event;
	type XorCurrency = Xor;
	type LockId = EthereumRelayAuthoritiesLockId;
	type TermDuration = EthereumRelayAuthoritiesTermDuration;
	type MaxCandidates = MaxCandidates;
	type AddOrigin = ApproveOrigin;
	type RemoveOrigin = ApproveOrigin;
	type ResetOrigin = ApproveOrigin;
	type OracolMMR = HeaderMMR;
	type Sign = EthereumBacking;
	type OpCodes = OpCodes;
	type SignThreshold = SignThreshold;
	type SubmitDuration = SubmitDuration;
	type WeightInfo = ();
}
