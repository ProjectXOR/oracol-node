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

//! # Oldxor Backing Module

#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	pub mod types {
		// --- oracol ---
		use super::*;

		// Generic types
		pub type AccountId<T> = <T as frame_system::Config>::AccountId;
		pub type XorBalance<T> = <XorCurrency<T> as Currency<AccountId<T>>>::Balance;
		type XorCurrency<T> = <T as Config>::XorCurrency;
	}
	pub use types::*;

	// --- substrate ---
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, Get},
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::{traits::AccountIdConversion, ModuleId};
	// --- oracol ---
	use crate::weights::WeightInfo;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		// --- substrate ---
		type WeightInfo: WeightInfo;
		// --- oracol ---
		#[pallet::constant]
		type ModuleId: Get<ModuleId>;
		type XorCurrency: Currency<AccountId<Self>>;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub backed_xor: XorBalance<T>,
	}
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				backed_xor: Default::default(),
			}
		}
	}
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			let _ = T::XorCurrency::make_free_balance_be(
				&T::ModuleId::get().into_account(),
				T::XorCurrency::minimum_balance() + self.backed_xor,
			);
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}
