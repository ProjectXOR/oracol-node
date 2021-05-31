// This file is part of Oracol.
//
// Copyright (C) 2018-2021 Oracol Network
// SPDX-License-Identifier: GPL-3.0
//
// Oracol is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Oracol is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Oracol. If not, see <https://www.gnu.org/licenses/>.

#![allow(dead_code)]

// --- crates ---
use codec::{Decode, Encode};
// --- substrate ---
use frame_support::traits::GenesisBuild;
use frame_system::mocking::*;
use sp_io::TestExternalities;
use sp_runtime::ModuleId;
use sp_runtime::{
	testing::{Header, H256},
	traits::{BlakeTwo256, IdentityLookup},
	RuntimeDebug,
};
// --- oracol ---
use crate::{self as oracol_oldxor_issuing, *};

// Global primitives
pub type Block = MockBlock<Test>;
pub type UncheckedExtrinsic = MockUncheckedExtrinsic<Test>;
pub type AccountId = u64;
pub type Balance = u128;
// Pallet primitives
pub type OldxorIssuingError = Error<Test>;

oracol_support::impl_test_account_data! {}

impl frame_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

frame_support::parameter_types! {
	pub const ExistentialDeposit: Balance = 0;
}
impl oracol_balances::Config<XorInstance> for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type BalanceInfo = AccountData<Balance>;
	type AccountStore = System;
	type MaxLocks = ();
	type OtherCurrencies = ();
	type WeightInfo = ();
}

frame_support::parameter_types! {
	pub const OldxorIssuingModuleId: ModuleId = ModuleId(*b"da/oldxori");
}
impl Config for Test {
	type Event = Event;
	type ModuleId = OldxorIssuingModuleId;
	type XorCurrency = Xor;
	type WeightInfo = ();
}

frame_support::construct_runtime! {
	pub enum Test
	where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Xor: oracol_balances::<Instance0>::{Pallet, Call, Storage, Config<T>, Event<T>},
		OracolOldxorIssuing: oracol_oldxor_issuing::{Pallet, Call, Storage, Config, Event<T>},
	}
}

pub fn new_test_ext() -> TestExternalities {
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();

	oracol_balances::GenesisConfig::<Test, XorInstance> {
		balances: (1..10)
			.map(|i: AccountId| vec![(i, 100 * i as Balance), (10 * i, 1000 * i as Balance)])
			.flatten()
			.collect(),
	}
	.assimilate_storage(&mut t)
	.unwrap();
	<oracol_oldxor_issuing::GenesisConfig as GenesisBuild<Test>>::assimilate_storage(
		&oracol_oldxor_issuing::GenesisConfig {
			total_mapped_xor: 4_000,
		},
		&mut t,
	)
	.unwrap();

	t.into()
}

pub fn events() -> Vec<Event> {
	let events = System::events()
		.into_iter()
		.map(|evt| evt.event)
		.collect::<Vec<_>>();

	System::reset_events();

	events
}

pub fn oldxor_issuing_events() -> Vec<Event> {
	events()
		.into_iter()
		.filter(|e| matches!(e, Event::oracol_oldxor_issuing(_)))
		.collect()
}
