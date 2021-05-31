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

mod util;

use codec::Decode;
use core::str::FromStr;
use ethabi::{Function, Param, ParamType, Token};
use evm::{Context, ExitError, ExitReason, ExitSucceed};
use frame_support::{ensure, traits::Currency};
use sha3::Digest;
use sp_core::{H160, U256};
use sp_runtime::{traits::UniqueSaturatedInto, SaturatedConversion};
use sp_std::{borrow::ToOwned, marker::PhantomData, prelude::*, vec::Vec};

use oracol_evm::{Account, AccountBasic, Config, Module, Runner};
use oracol_support::evm::POW_9;
use dp_evm::Precompile;
use dvm_ethereum::{
	account_basic::{OxorRemainBalance, RemainBalanceOp},
	OxorBalance,
};

type AccountId<T> = <T as frame_system::Config>::AccountId;

const TRANSFER_AND_CALL_ACTION: &[u8] = b"transfer_and_call(address,uint256)";
const WITHDRAW_ACTION: &[u8] = b"withdraw(bytes32,uint256)";
const OXOR_PRECOMPILE: &str = "0000000000000000000000000000000000000016";
/// Oxor Precompile Contract is used to support the exchange of OXOR native asset between oracol and dvm contract
///
/// The contract address: 0000000000000000000000000000000000000016
pub struct Oxor<T: Config> {
	_maker: PhantomData<T>,
}

impl<T: Config + dvm_ethereum::Config> Precompile for Oxor<T> {
	/// There are two actions, one is `transfer_and_call` and the other is `withdraw`
	/// 1. Transfer_and_call Action, triggered by the user sending a transaction to the oxor precompile
	/// 	   special evm address, eg(0000000000000000000000000000000000000016). and transfer the sender's
	///     oxor balance to the deployed woxor contract in dvm. The input contain two parts:
	///     - p1: The woxor address, it is important to note that if this address is wrong, the balance cannot be recovered.
	///     - p2: The transfer value
	/// 2. WithDraw Action, the user sends transaction to woxor contract triggering this oxor precompile to be called
	///     within the woxor contract, and transfer the balance from woxor balanceof to the oracol network. The input contain two parts:
	///     - p1: The to account id, a withdraw oracol public key.
	///     - p2: The withdraw value
	fn execute(
		input: &[u8],
		target_limit: Option<u64>,
		context: &Context,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, u64), ExitError> {
		let helper = U256::from(POW_9);
		let action = which_action::<T>(&input)?;

		match action {
			Action::TransferAndCall(call_data) => {
				// Ensure woxor is a contract
				ensure!(
					!crate::Module::<T>::is_contract_code_empty(&call_data.woxor_address),
					ExitError::Other("Woxor must be a contract!".into())
				);
				// Ensure context's apparent_value is zero, since the transfer value is encoded in input field
				ensure!(
					context.apparent_value == U256::zero(),
					ExitError::Other("The value in tx must be zero!".into())
				);
				// Ensure caller's balance is enough
				ensure!(
					T::OxorAccountBasic::account_basic(&context.caller).balance >= call_data.value,
					ExitError::OutOfFund
				);

				// Transfer oxor from sender to OXOR wrapped contract
				T::OxorAccountBasic::transfer(
					&context.caller,
					&call_data.woxor_address,
					call_data.value,
				)?;
				// Call WOXOR wrapped contract deposit
				let precompile_address = H160::from_str(OXOR_PRECOMPILE).unwrap_or_default();
				let raw_input = make_call_data(context.caller, call_data.value)?;
				if let Ok(call_res) = T::Runner::call(
					precompile_address,
					call_data.woxor_address,
					raw_input.to_vec(),
					U256::zero(),
					target_limit.unwrap_or_default(),
					None,
					None,
					T::config(),
				) {
					match call_res.exit_reason {
						ExitReason::Succeed(_) => {
							log::debug!("Transfer and call execute success.");
						}
						_ => return Err(ExitError::Other("Call in Oxor precompile failed".into())),
					}
				}

				Ok((ExitSucceed::Returned, vec![], 20000))
			}
			Action::Withdraw(wd) => {
				// Ensure woxor is a contract
				ensure!(
					!crate::Module::<T>::is_contract_code_empty(&context.caller),
					ExitError::Other("The caller must be woxor contract!".into())
				);
				// Ensure context's apparent_value is zero
				ensure!(
					context.apparent_value == U256::zero(),
					ExitError::Other("The value in tx must be zero!".into())
				);
				// Ensure caller's balance is enough
				let caller_oxor = T::OxorAccountBasic::account_basic(&context.caller);
				ensure!(caller_oxor.balance >= wd.oxor_value, ExitError::OutOfFund);

				// Transfer
				let new_woxor_balance = caller_oxor.balance.saturating_sub(wd.oxor_value);
				T::OxorAccountBasic::mutate_account_basic(
					&context.caller,
					Account {
						nonce: caller_oxor.nonce,
						balance: new_woxor_balance,
					},
				);
				let (currency_value, remain_balance) = wd.oxor_value.div_mod(helper);
				<T as oracol_evm::Config>::OxorCurrency::deposit_creating(
					&wd.to_account_id,
					currency_value.low_u128().unique_saturated_into(),
				);
				<OxorRemainBalance as RemainBalanceOp<T, OxorBalance<T>>>::inc_remaining_balance(
					&wd.to_account_id,
					remain_balance.low_u128().saturated_into(),
				);

				Ok((ExitSucceed::Returned, vec![], 20000))
			}
		}
	}
}

/// Action about OXOR precompile
pub enum Action<T: frame_system::Config> {
	/// Transfer from substrate account to woxor contract
	TransferAndCall(CallData),
	/// Withdraw from woxor contract to substrate account
	Withdraw(WithdrawData<T>),
}

/// which action depends on the function selector
pub fn which_action<T: frame_system::Config>(input_data: &[u8]) -> Result<Action<T>, ExitError> {
	let transfer_and_call_action = &sha3::Keccak256::digest(&TRANSFER_AND_CALL_ACTION)[0..4];
	let withdraw_action = &sha3::Keccak256::digest(&WITHDRAW_ACTION)[0..4];
	if &input_data[0..4] == transfer_and_call_action {
		let decoded_data = CallData::decode(&input_data[4..])?;
		return Ok(Action::TransferAndCall(decoded_data));
	} else if &input_data[0..4] == withdraw_action {
		let decoded_data = WithdrawData::decode(&input_data[4..])?;
		return Ok(Action::Withdraw(decoded_data));
	}
	Err(ExitError::Other("Invalid Action！".into()))
}

#[derive(Debug, PartialEq, Eq)]
pub struct CallData {
	woxor_address: H160,
	value: U256,
}

impl CallData {
	pub fn decode(data: &[u8]) -> Result<Self, ExitError> {
		let tokens = ethabi::decode(&[ParamType::Address, ParamType::Uint(256)], &data)
			.map_err(|_| ExitError::Other("ethabi decoded error".into()))?;
		match (tokens[0].clone(), tokens[1].clone()) {
			(Token::Address(eth_woxor_address), Token::Uint(eth_value)) => Ok(CallData {
				woxor_address: util::e2s_address(eth_woxor_address),
				value: util::e2s_u256(eth_value),
			}),
			_ => Err(ExitError::Other("Invlid call data".into())),
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct WithdrawData<T: frame_system::Config> {
	pub to_account_id: AccountId<T>,
	pub oxor_value: U256,
}

impl<T: frame_system::Config> WithdrawData<T> {
	pub fn decode(data: &[u8]) -> Result<Self, ExitError> {
		let tokens = ethabi::decode(&[ParamType::FixedBytes(32), ParamType::Uint(256)], &data)
			.map_err(|_| ExitError::Other("ethabi decoded error".into()))?;
		match (tokens[0].clone(), tokens[1].clone()) {
			(Token::FixedBytes(address), Token::Uint(eth_value)) => Ok(WithdrawData {
				to_account_id: <T as frame_system::Config>::AccountId::decode(
					&mut address.as_ref(),
				)
				.map_err(|_| ExitError::Other("Invalid destination address".into()))?,
				oxor_value: util::e2s_u256(eth_value),
			}),
			_ => Err(ExitError::Other("Invlid withdraw input data".into())),
		}
	}
}

fn make_call_data(
	sp_address: sp_core::H160,
	sp_value: sp_core::U256,
) -> Result<Vec<u8>, ExitError> {
	let eth_address = util::s2e_address(sp_address);
	let eth_value = util::s2e_u256(sp_value);
	let func = Function {
		name: "deposit".to_owned(),
		inputs: vec![
			Param {
				name: "address".to_owned(),
				kind: ParamType::Address,
			},
			Param {
				name: "value".to_owned(),
				kind: ParamType::Uint(256),
			},
		],
		outputs: vec![],
		constant: false,
	};
	func.encode_input(&[Token::Address(eth_address), Token::Uint(eth_value)])
		.map_err(|_| ExitError::Other("Make call data error happened".into()))
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::str::FromStr;

	#[test]
	fn test_make_input() {
		let mock_address =
			sp_core::H160::from_str("Aa01a1bEF0557fa9625581a293F3AA7770192632").unwrap();
		let mock_value_1 = sp_core::U256::from(30);
		let expected_str = "0x47e7ef24000000000000000000000000aa01a1bef0557fa9625581a293f3aa7770192632000000000000000000000000000000000000000000000000000000000000001e";

		let encoded_str =
			array_bytes::bytes2hex("0x", make_call_data(mock_address, mock_value_1).unwrap());
		assert_eq!(encoded_str, expected_str);

		let mock_value_2 = sp_core::U256::from(25);
		let encoded_str =
			array_bytes::bytes2hex("0x", make_call_data(mock_address, mock_value_2).unwrap());
		assert_ne!(encoded_str, expected_str);
	}
}