// --- oracol ---
pub use oracol_balances::{Instance0 as XorInstance, Instance1 as OxorInstance};

// --- substrate ---
use frame_support::traits::Currency;
use frame_system::Config as SystemConfig;
// --- oracol ---
use crate::*;
use oracol_balances::{weights::SubstrateWeight, Config, Pallet};

pub type XorNegativeImbalance = <Pallet<Runtime, XorInstance> as Currency<
	<Runtime as SystemConfig>::AccountId,
>>::NegativeImbalance;

frame_support::parameter_types! {
	pub const ExistentialDeposit: Balance = 0;
	pub const MaxLocks: u32 = 50;
}
impl Config<XorInstance> for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type BalanceInfo = AccountData<Balance>;
	type AccountStore = System;
	type MaxLocks = MaxLocks;
	type OtherCurrencies = (Oxor,);
	type WeightInfo = SubstrateWeight<Runtime>;
}
impl Config<OxorInstance> for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type BalanceInfo = AccountData<Balance>;
	type AccountStore = System;
	type MaxLocks = MaxLocks;
	type OtherCurrencies = (Xor,);
	type WeightInfo = SubstrateWeight<Runtime>;
}
