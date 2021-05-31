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
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Oracol. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// --- core ---
use core::marker::PhantomData;
// --- alloc ---
use alloc::vec::Vec;
// --- crates ---
use evm::{Context, ExitError, ExitSucceed};
// --- oracol ---
use oracol_evm::{Config, IssuingHandler};
use dp_evm::Precompile;

/// Issuing Precompile Contract, used to burn mapped token and generate a event proof in oracol
///
/// The contract address: 0000000000000000000000000000000000000017
pub struct Issuing<T: Config> {
	_maker: PhantomData<T>,
}
impl<T: Config> Precompile for Issuing<T> {
	fn execute(
		input: &[u8],
		_: Option<u64>,
		context: &Context,
	) -> Result<(ExitSucceed, Vec<u8>, u64), ExitError> {
		T::IssuingHandler::handle(context.address, context.caller, input)
			.map_err(|_| ExitError::Other("contract handle failed".into()))?;
		Ok((ExitSucceed::Returned, Default::default(), 20000))
	}
}
