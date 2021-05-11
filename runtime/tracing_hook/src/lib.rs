// Copyright 2019-2020 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Substrate EVM tracing.
//!
//! The purpose of this crate is enable tracing the EVM opcode execution and will be used by
//! both Dapp developers - to get a granular view on their transactions - and indexers to access
//! the EVM callstack (internal transactions).
//!
//! This crate provide a Hook with 2 modes :
//!
//! - Raw : allows capturing the intermediate machine
//!   state between opcode executions (stepping), resulting in either a granular per opcode response:
//!   ```json
//!   {
//!     "pc": 230,
//!     "op": "SSTORE",
//!     "gas": 62841,
//!     "gasCost": 20000,
//!     "depth": 1,
//!     "stack": [
//!       "00000000000000000000000000000000000000000000000000000000398f7223",
//!     ],
//!     "memory": [
//!       "0000000000000000000000000000000000000000000000000000000000000000",
//!     ],
//!     "storage": {"0x":"0x"}
//!   }
//!   ```
//! - Call list : overview of the internal transactions in a context type :
//!   ```json
//!   [
//!    {
//!      "type": "call",
//!      "callType": "call",
//!      "from": "0xfe2882ac0a337a976aa73023c2a2a917f57ba2ed",
//!      "to": "0x3ca17a1c4995b95c600275e52da93d2e64dd591f",
//!      "input": "0x",
//!      "output": "0x",
//!      "traceAddress": [],
//!      "value": "0x0",
//!      "gas": "0xf9be",
//!      "gasUsed": "0xf9be"
//!    },
//!    {
//!      "type": "call",
//!      "callType": "call",
//!      "from": "0x3ca17a1c4995b95c600275e52da93d2e64dd591f",
//!      "to": "0x1416aa2a27db08ce3a81a01cdfc981417d28a6e6",
//!      "input": "0xfd63983b0000000000000000000000000000000000000000000000000000000000000006",
//!      "output": "0x000000000000000000000000000000000000000000000000000000000000000d",
//!      "traceAddress": [0],
//!      "value": "0x0",
//!      "gas": "0x9b9b",
//!      "gasUsed": "0x4f6d"
//!    }
//!   ]
//!   ```

use evm_gasometer::tracing; // working use !

// #![cfg_attr(not(feature = "std"), no_std)]

// use evm::{
// 	executor::{Hook, HookSite, StackExecutor, StackState},
// 	Config, ExitReason, Opcode, Runtime,
// };
// use moonbeam_rpc_primitives_debug::single::TransactionTrace;
// use sp_std::vec::Vec;

// mod call_list;
// mod raw;

// pub struct TracingHook(TracingHookInner);

// impl TracingHook {
// 	pub fn new_raw(disable_storage: bool, disable_memory: bool, disable_stack: bool) -> Self {
// 		Self(TracingHookInner::Raw(raw::State::new(
// 			disable_storage,
// 			disable_memory,
// 			disable_stack,
// 		)))
// 	}

// 	pub fn new_call_list() -> Self {
// 		Self(TracingHookInner::CallList(call_list::State::new()))
// 	}

// 	pub fn finish(self) -> TransactionTrace {
// 		match self.0 {
// 			TracingHookInner::Raw(state) => state.finish(),
// 			TracingHookInner::CallList(state) => state.finish(),
// 		}
// 	}
// }

// enum TracingHookInner {
// 	Raw(raw::State),
// 	CallList(call_list::State),
// }

// impl Hook for TracingHook {
// 	fn hook<'config, S: StackState<'config>>(
// 		&mut self,
// 		site: HookSite,
// 		config: &'config Config,
// 		state: &S,
// 		runtime: &Runtime,
// 	) {
// 		match &mut self.0 {
// 			TracingHookInner::Raw(inner) => inner.hook(site, config, state, runtime),
// 			TracingHookInner::CallList(inner) => inner.hook(site, config, state, runtime),
// 		}
// 	}
// }

// fn opcodes(opcode: Opcode) -> Vec<u8> {
// 	let out = match opcode {
// 		Opcode(0) => "Stop",
// 		Opcode(1) => "Add",
// 		Opcode(2) => "Mul",
// 		Opcode(3) => "Sub",
// 		Opcode(4) => "Div",
// 		Opcode(5) => "SDiv",
// 		Opcode(6) => "Mod",
// 		Opcode(7) => "SMod",
// 		Opcode(8) => "AddMod",
// 		Opcode(9) => "MulMod",
// 		Opcode(10) => "Exp",
// 		Opcode(11) => "SignExtend",
// 		Opcode(16) => "Lt",
// 		Opcode(17) => "Gt",
// 		Opcode(18) => "Slt",
// 		Opcode(19) => "Sgt",
// 		Opcode(20) => "Eq",
// 		Opcode(21) => "IsZero",
// 		Opcode(22) => "And",
// 		Opcode(23) => "Or",
// 		Opcode(24) => "Xor",
// 		Opcode(25) => "Not",
// 		Opcode(26) => "Byte",
// 		Opcode(27) => "Shl",
// 		Opcode(28) => "Shr",
// 		Opcode(29) => "Sar",
// 		Opcode(32) => "Keccak256",
// 		Opcode(48) => "Address",
// 		Opcode(49) => "Balance",
// 		Opcode(50) => "Origin",
// 		Opcode(51) => "Caller",
// 		Opcode(52) => "CallValue",
// 		Opcode(53) => "CallDataLoad",
// 		Opcode(54) => "CallDataSize",
// 		Opcode(55) => "CallDataCopy",
// 		Opcode(56) => "CodeSize",
// 		Opcode(57) => "CodeCopy",
// 		Opcode(58) => "GasPrice",
// 		Opcode(59) => "ExtCodeSize",
// 		Opcode(60) => "ExtCodeCopy",
// 		Opcode(61) => "ReturnDataSize",
// 		Opcode(62) => "ReturnDataCopy",
// 		Opcode(63) => "ExtCodeHash",
// 		Opcode(64) => "BlockHash",
// 		Opcode(65) => "Coinbase",
// 		Opcode(66) => "Timestamp",
// 		Opcode(67) => "Number",
// 		Opcode(68) => "Difficulty",
// 		Opcode(69) => "GasLimit",
// 		Opcode(70) => "ChainId",
// 		Opcode(80) => "Pop",
// 		Opcode(81) => "MLoad",
// 		Opcode(82) => "MStore",
// 		Opcode(83) => "MStore8",
// 		Opcode(84) => "SLoad",
// 		Opcode(85) => "SStore",
// 		Opcode(86) => "Jump",
// 		Opcode(87) => "JumpI",
// 		Opcode(88) => "GetPc",
// 		Opcode(89) => "MSize",
// 		Opcode(90) => "Gas",
// 		Opcode(91) => "JumpDest",
// 		Opcode(96) => "Push1",
// 		Opcode(97) => "Push2",
// 		Opcode(98) => "Push3",
// 		Opcode(99) => "Push4",
// 		Opcode(100) => "Push5",
// 		Opcode(101) => "Push6",
// 		Opcode(102) => "Push7",
// 		Opcode(103) => "Push8",
// 		Opcode(104) => "Push9",
// 		Opcode(105) => "Push10",
// 		Opcode(106) => "Push11",
// 		Opcode(107) => "Push12",
// 		Opcode(108) => "Push13",
// 		Opcode(109) => "Push14",
// 		Opcode(110) => "Push15",
// 		Opcode(111) => "Push16",
// 		Opcode(112) => "Push17",
// 		Opcode(113) => "Push18",
// 		Opcode(114) => "Push19",
// 		Opcode(115) => "Push20",
// 		Opcode(116) => "Push21",
// 		Opcode(117) => "Push22",
// 		Opcode(118) => "Push23",
// 		Opcode(119) => "Push24",
// 		Opcode(120) => "Push25",
// 		Opcode(121) => "Push26",
// 		Opcode(122) => "Push27",
// 		Opcode(123) => "Push28",
// 		Opcode(124) => "Push29",
// 		Opcode(125) => "Push30",
// 		Opcode(126) => "Push31",
// 		Opcode(127) => "Push32",
// 		Opcode(128) => "Dup1",
// 		Opcode(129) => "Dup2",
// 		Opcode(130) => "Dup3",
// 		Opcode(131) => "Dup4",
// 		Opcode(132) => "Dup5",
// 		Opcode(133) => "Dup6",
// 		Opcode(134) => "Dup7",
// 		Opcode(135) => "Dup8",
// 		Opcode(136) => "Dup9",
// 		Opcode(137) => "Dup10",
// 		Opcode(138) => "Dup11",
// 		Opcode(139) => "Dup12",
// 		Opcode(140) => "Dup13",
// 		Opcode(141) => "Dup14",
// 		Opcode(142) => "Dup15",
// 		Opcode(143) => "Dup16",
// 		Opcode(144) => "Swap1",
// 		Opcode(145) => "Swap2",
// 		Opcode(146) => "Swap3",
// 		Opcode(147) => "Swap4",
// 		Opcode(148) => "Swap5",
// 		Opcode(149) => "Swap6",
// 		Opcode(150) => "Swap7",
// 		Opcode(151) => "Swap8",
// 		Opcode(152) => "Swap9",
// 		Opcode(153) => "Swap10",
// 		Opcode(154) => "Swap11",
// 		Opcode(155) => "Swap12",
// 		Opcode(156) => "Swap13",
// 		Opcode(157) => "Swap14",
// 		Opcode(158) => "Swap15",
// 		Opcode(159) => "Swap16",
// 		Opcode(160) => "Log0",
// 		Opcode(161) => "Log1",
// 		Opcode(162) => "Log2",
// 		Opcode(163) => "Log3",
// 		Opcode(164) => "Log4",
// 		Opcode(176) => "JumpTo",
// 		Opcode(177) => "JumpIf",
// 		Opcode(178) => "JumpSub",
// 		Opcode(180) => "JumpSubv",
// 		Opcode(181) => "BeginSub",
// 		Opcode(182) => "BeginData",
// 		Opcode(184) => "ReturnSub",
// 		Opcode(185) => "PutLocal",
// 		Opcode(186) => "GetLocal",
// 		Opcode(225) => "SLoadBytes",
// 		Opcode(226) => "SStoreBytes",
// 		Opcode(227) => "SSize",
// 		Opcode(240) => "Create",
// 		Opcode(241) => "Call",
// 		Opcode(242) => "CallCode",
// 		Opcode(243) => "Return",
// 		Opcode(244) => "DelegateCall",
// 		Opcode(245) => "Create2",
// 		Opcode(250) => "StaticCall",
// 		Opcode(252) => "TxExecGas",
// 		Opcode(253) => "Revert",
// 		Opcode(254) => "Invalid",
// 		Opcode(255) => "SelfDestruct",
// 		_ => unreachable!("Unreachable Opcode identifier."),
// 	};
// 	out.as_bytes().to_vec()
// }
