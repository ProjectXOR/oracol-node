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

//! # Oldoxor Backing Module

#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;
// --- oracol ---
pub use weights::WeightInfo;

mod types {
	// --- oracol ---
	#[cfg(feature = "std")]
	use crate::*;

	pub type AccountId<T> = <T as frame_system::Config>::AccountId;

	#[cfg(feature = "std")]
	pub type XorBalance<T> = <XorCurrency<T> as Currency<AccountId<T>>>::Balance;
	#[cfg(feature = "std")]
	pub type OxorBalance<T> = <OxorCurrency<T> as Currency<AccountId<T>>>::Balance;

	#[cfg(feature = "std")]
	type XorCurrency<T> = <T as Config>::XorCurrency;
	#[cfg(feature = "std")]
	type OxorCurrency<T> = <T as Config>::OxorCurrency;
}

// --- substrate ---
use frame_support::{
	decl_module, decl_storage,
	traits::{Currency, Get},
};
use sp_runtime::{traits::AccountIdConversion, ModuleId};
// --- oracol ---
use types::*;

pub trait Config: frame_system::Config {
	type ModuleId: Get<ModuleId>;

	type XorCurrency: Currency<AccountId<Self>>;
	type OxorCurrency: Currency<AccountId<Self>>;

	type WeightInfo: WeightInfo;
}

decl_storage! {
	trait Store for Module<T: Config> as OracolOldoxorBacking {}

	add_extra_genesis {
		config(backed_xor): XorBalance<T>;
		config(backed_oxor): OxorBalance<T>;
		build(|config| {
			let module_account = <Module<T>>::account_id();
			let _ = T::XorCurrency::make_free_balance_be(
				&module_account,
				T::XorCurrency::minimum_balance() + config.backed_xor
			);
			let _ = T::OxorCurrency::make_free_balance_be(
				&module_account,
				config.backed_oxor
			);
		});
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call
	where
		origin: T::Origin
	{
		const ModuleId: ModuleId = T::ModuleId::get();
	}
}

impl<T: Config> Module<T> {
	pub fn account_id() -> T::AccountId {
		T::ModuleId::get().into_account()
	}
}
