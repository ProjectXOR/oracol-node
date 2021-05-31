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

#[macro_export]
macro_rules! impl_test_account_data {
	() => {
		pub type XorInstance = oracol_balances::Instance0;
		pub type XorError = oracol_balances::Error<Test, XorInstance>;
		pub type OxorInstance = oracol_balances::Instance1;
		pub type OxorError = oracol_balances::Error<Test, OxorInstance>;

		$crate::impl_account_data! {
			struct AccountData<Balance>
			for
				XorInstance,
				OxorInstance
			where
				Balance = Balance
			{}
		}
	};
}
