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

//! # Oldxor Issuing Module

#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	pub mod types {
		// --- oracol ---
		use super::*;

		// Generic type
		pub type AccountId<T> = <T as frame_system::Config>::AccountId;
		pub type XorBalance<T> = <XorCurrency<T> as Currency<AccountId<T>>>::Balance;
		type XorCurrency<T> = <T as Config>::XorCurrency;
		// Simple type
		pub type MappedXor = u128;
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
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type WeightInfo: WeightInfo;
		// --- oracol ---
		#[pallet::constant]
		type ModuleId: Get<ModuleId>;
		type XorCurrency: Currency<AccountId<Self>>;
	}

	#[pallet::event]
	pub enum Event<T: Config> {
		/// Dummy Event. \[who, swapped *CXOR*, burned Mapped *XOR*\]
		DummyEvent(AccountId<T>, XorBalance<T>, MappedXor),
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::storage]
	#[pallet::getter(fn total_mapped_xor)]
	pub type TotalMappedXor<T: Config> = StorageValue<_, MappedXor>;

	#[cfg_attr(feature = "std", derive(Default))]
	#[pallet::genesis_config]
	pub struct GenesisConfig {
		pub total_mapped_xor: MappedXor,
	}
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			let _ = T::XorCurrency::make_free_balance_be(
				&T::ModuleId::get().into_account(),
				T::XorCurrency::minimum_balance(),
			);

			<TotalMappedXor<T>>::put(self.total_mapped_xor);
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}

pub mod migration {
	const OLD_PALLET_NAME: &[u8] = b"OracolOldxorIssuing";

	#[cfg(feature = "try-runtime")]
	pub mod try_runtime {
		// --- substrate ---
		use frame_support::{pallet_prelude::*, traits::StorageInstance};
		// --- oracol ---
		use crate::*;

		macro_rules! generate_storage_types {
			($prefix:expr, $name:ident => Value<$value:ty>) => {
				paste::paste! {
					type $name = StorageValue<[<$name Instance>], $value, ValueQuery>;

					struct [<$name Instance>];
					impl StorageInstance for [<$name Instance>] {
						const STORAGE_PREFIX: &'static str = "TotalMappedXor";

						fn pallet_prefix() -> &'static str { $prefix }
					}
				}
			};
		}

		generate_storage_types!("OracolOldxorIssuing", OldTotalMappedXor => Value<()>);
		generate_storage_types!("OldxorIssuing", NewTotalMappedXor => Value<()>);

		pub fn pre_migrate<T: Config>() -> Result<(), &'static str> {
			log::info!(
				"OldTotalMappedXor.exits()? {:?}",
				OldTotalMappedXor::exists()
			);
			log::info!(
				"NewTotalMappedXor.exits()? {:?}",
				NewTotalMappedXor::exists()
			);

			assert!(OldTotalMappedXor::exists());
			assert!(!NewTotalMappedXor::exists());

			log::info!("Migrating `OracolOldxorIssuing` to `OldxorIssuing`...");
			migration::migrate(b"OldxorIssuing");

			log::info!(
				"OldTotalMappedXor.exits()? {:?}",
				OldTotalMappedXor::exists()
			);
			log::info!(
				"NewTotalMappedXor.exits()? {:?}",
				NewTotalMappedXor::exists()
			);

			assert!(!OldTotalMappedXor::exists());
			assert!(NewTotalMappedXor::exists());

			Ok(())
		}
	}

	pub fn migrate(new_pallet_name: &[u8]) {
		frame_support::migration::move_pallet(OLD_PALLET_NAME, new_pallet_name);
	}
}
