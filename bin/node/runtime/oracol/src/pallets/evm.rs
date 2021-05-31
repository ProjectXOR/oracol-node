// --- substrate ---
use sp_core::U256;
// --- oracol ---
use crate::*;
use oracol_evm::{
	runner::stack::Runner, ConcatAddressMapping, Config, EnsureAddressTruncated, FeeCalculator,
};
use oracol_evm_precompile::OracolPrecompiles;
use dvm_ethereum::account_basic::DvmAccountBasic;
use dvm_ethereum::account_basic::{OxorRemainBalance, XorRemainBalance};

/// Fixed gas price.
pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
	fn min_gas_price() -> U256 {
		// Gas price is always one token per gas.
		1_000_000_000.into()
	}
}
frame_support::parameter_types! {
	pub const ChainId: u64 = 43;
	pub BlockGasLimit: U256 = U256::from(u32::max_value());
}
impl Config for Runtime {
	type FeeCalculator = FixedGasPrice;
	type GasWeightMapping = ();
	type CallOrigin = EnsureAddressTruncated;
	type WithdrawOrigin = EnsureAddressTruncated;
	type AddressMapping = ConcatAddressMapping;
	type XorCurrency = Xor;
	type OxorCurrency = Oxor;
	type Event = Event;
	type Precompiles = OracolPrecompiles<Self>;
	type ChainId = ChainId;
	type BlockGasLimit = BlockGasLimit;
	type XorAccountBasic = DvmAccountBasic<Self, Xor, XorRemainBalance>;
	type OxorAccountBasic = DvmAccountBasic<Self, Oxor, OxorRemainBalance>;
	type Runner = Runner<Self>;
	type IssuingHandler = EthereumIssuing;
}
