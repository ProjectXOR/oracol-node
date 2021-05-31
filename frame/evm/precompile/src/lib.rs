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

pub type OracolPrecompiles<Runtime> = (
	oracol_evm_precompile_simple::ECRecover, // 0x0000000000000000000000000000000000000001
	oracol_evm_precompile_simple::Sha256,    // 0x0000000000000000000000000000000000000002
	oracol_evm_precompile_simple::Ripemd160, // 0x0000000000000000000000000000000000000003
	oracol_evm_precompile_simple::Identity,  // 0x0000000000000000000000000000000000000004
	oracol_evm_precompile_empty::Empty,      // 0x0000000000000000000000000000000000000005
	oracol_evm_precompile_empty::Empty,      // 0x0000000000000000000000000000000000000006
	oracol_evm_precompile_empty::Empty,      // 0x0000000000000000000000000000000000000007
	oracol_evm_precompile_empty::Empty,      // 0x0000000000000000000000000000000000000008
	oracol_evm_precompile_empty::Empty,      // 0x0000000000000000000000000000000000000009
	oracol_evm_precompile_empty::Empty,      // 0x000000000000000000000000000000000000000a
	oracol_evm_precompile_empty::Empty,      // 0x000000000000000000000000000000000000000b
	oracol_evm_precompile_empty::Empty,      // 0x000000000000000000000000000000000000000c
	oracol_evm_precompile_empty::Empty,      // 0x000000000000000000000000000000000000000d
	oracol_evm_precompile_empty::Empty,      // 0x000000000000000000000000000000000000000e
	oracol_evm_precompile_empty::Empty,      // 0x000000000000000000000000000000000000000f
	oracol_evm_precompile_empty::Empty,      // 0x0000000000000000000000000000000000000010
	oracol_evm_precompile_empty::Empty,      // 0x0000000000000000000000000000000000000011
	oracol_evm_precompile_empty::Empty,      // 0x0000000000000000000000000000000000000012
	oracol_evm_precompile_empty::Empty,      // 0x0000000000000000000000000000000000000013
	oracol_evm_precompile_empty::Empty,      // 0x0000000000000000000000000000000000000014
	oracol_evm_precompile_withdraw::WithDraw<Runtime>, // 0x0000000000000000000000000000000000000015
	oracol_evm_precompile_oxor::Oxor<Runtime>, // 0x0000000000000000000000000000000000000016
	oracol_evm_precompile_issuing::Issuing<Runtime>, // 0x0000000000000000000000000000000000000017
);
