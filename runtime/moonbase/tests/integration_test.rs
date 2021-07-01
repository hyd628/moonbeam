// Copyright 2019-2021 PureStake Inc.
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

//! Moonbase Runtime Integration Tests

mod common;
use common::*;

use evm::{executor::PrecompileOutput, ExitError, ExitSucceed};
use frame_support::{
	assert_noop, assert_ok,
	dispatch::Dispatchable,
	traits::{fungible::Inspect, PalletInfo},
};
use moonbase_runtime::{
	currency::UNITS, AccountId, Balances, Call, CrowdloanRewards, Event, ParachainStaking,
	Precompiles, Runtime, System,
};
use nimbus_primitives::NimbusId;
use pallet_evm::PrecompileSet;
use parachain_staking::{Bond, NominatorAdded};
use sp_core::{Public, H160, U256};
use sp_runtime::DispatchError;

#[test]
fn verify_pallet_indices() {
	fn is_pallet_index<P: 'static>(index: usize) {
		assert_eq!(
			<moonbase_runtime::Runtime as frame_system::Config>::PalletInfo::index::<P>(),
			Some(index)
		);
	}
	is_pallet_index::<moonbase_runtime::System>(0);
	is_pallet_index::<moonbase_runtime::Utility>(1);
	is_pallet_index::<moonbase_runtime::Timestamp>(2);
	is_pallet_index::<moonbase_runtime::Balances>(3);
	is_pallet_index::<moonbase_runtime::Sudo>(4);
	is_pallet_index::<moonbase_runtime::RandomnessCollectiveFlip>(5);
	is_pallet_index::<moonbase_runtime::ParachainSystem>(6);
	is_pallet_index::<moonbase_runtime::TransactionPayment>(7);
	is_pallet_index::<moonbase_runtime::ParachainInfo>(8);
	is_pallet_index::<moonbase_runtime::EthereumChainId>(9);
	is_pallet_index::<moonbase_runtime::EVM>(10);
	is_pallet_index::<moonbase_runtime::Ethereum>(11);
	is_pallet_index::<moonbase_runtime::ParachainStaking>(12);
	is_pallet_index::<moonbase_runtime::Scheduler>(13);
	is_pallet_index::<moonbase_runtime::Democracy>(14);
	is_pallet_index::<moonbase_runtime::CouncilCollective>(15);
	is_pallet_index::<moonbase_runtime::TechComitteeCollective>(16);
	is_pallet_index::<moonbase_runtime::Treasury>(17);
	is_pallet_index::<moonbase_runtime::AuthorInherent>(18);
	is_pallet_index::<moonbase_runtime::AuthorFilter>(19);
	is_pallet_index::<moonbase_runtime::CrowdloanRewards>(20);
	is_pallet_index::<moonbase_runtime::AuthorMapping>(21);
	is_pallet_index::<moonbase_runtime::Proxy>(22);
}

#[test]
fn join_collator_candidates() {
	ExtBuilder::default()
		.with_balances(vec![
			(AccountId::from(ALICE), 2_000 * UNITS),
			(AccountId::from(BOB), 2_000 * UNITS),
			(AccountId::from(CHARLIE), 1_100 * UNITS),
			(AccountId::from(DAVE), 1_000 * UNITS),
		])
		.with_collators(vec![
			(AccountId::from(ALICE), 1_000 * UNITS),
			(AccountId::from(BOB), 1_000 * UNITS),
		])
		.with_nominations(vec![
			(AccountId::from(CHARLIE), AccountId::from(ALICE), 50 * UNITS),
			(AccountId::from(CHARLIE), AccountId::from(BOB), 50 * UNITS),
		])
		.build()
		.execute_with(|| {
			assert_noop!(
				ParachainStaking::join_candidates(
					origin_of(AccountId::from(ALICE)),
					1_000 * UNITS,
					2u32
				),
				parachain_staking::Error::<Runtime>::CandidateExists
			);
			assert_noop!(
				ParachainStaking::join_candidates(
					origin_of(AccountId::from(CHARLIE)),
					1_000 * UNITS,
					2u32
				),
				parachain_staking::Error::<Runtime>::NominatorExists
			);
			assert!(System::events().is_empty());
			assert_ok!(ParachainStaking::join_candidates(
				origin_of(AccountId::from(DAVE)),
				1_000 * UNITS,
				2u32
			));
			assert_eq!(
				last_event(),
				Event::ParachainStaking(parachain_staking::Event::JoinedCollatorCandidates(
					AccountId::from(DAVE),
					1_000 * UNITS,
					3_100 * UNITS
				))
			);
			let candidates = ParachainStaking::candidate_pool();
			assert_eq!(
				candidates.0[0],
				Bond {
					owner: AccountId::from(ALICE),
					amount: 1_050 * UNITS
				}
			);
			assert_eq!(
				candidates.0[1],
				Bond {
					owner: AccountId::from(BOB),
					amount: 1_050 * UNITS
				}
			);
			assert_eq!(
				candidates.0[2],
				Bond {
					owner: AccountId::from(DAVE),
					amount: 1_000 * UNITS
				}
			);
		});
}

#[test]
fn transfer_through_evm_to_stake() {
	ExtBuilder::default()
		.with_balances(vec![(AccountId::from(ALICE), 2_000 * UNITS)])
		.build()
		.execute_with(|| {
			// Charlie has no balance => fails to stake
			assert_noop!(
				ParachainStaking::join_candidates(
					origin_of(AccountId::from(CHARLIE)),
					1_000 * UNITS,
					0u32
				),
				DispatchError::Module {
					index: 3,
					error: 2,
					message: Some("InsufficientBalance")
				}
			);

			// Alice transfer from free balance 2000 UNITS to Bob
			assert_ok!(Balances::transfer(
				origin_of(AccountId::from(ALICE)),
				AccountId::from(BOB),
				2_000 * UNITS,
			));
			assert_eq!(Balances::free_balance(AccountId::from(BOB)), 2_000 * UNITS);

			let gas_limit = 100000u64;
			let gas_price: U256 = 1_000_000_000.into();
			// Bob transfers 1000 UNITS to Charlie via EVM
			assert_ok!(Call::EVM(pallet_evm::Call::<Runtime>::call(
				AccountId::from(BOB),
				AccountId::from(CHARLIE),
				Vec::new(),
				(1_000 * UNITS).into(),
				gas_limit,
				gas_price,
				None
			))
			.dispatch(<Runtime as frame_system::Config>::Origin::root()));
			assert_eq!(
				Balances::free_balance(AccountId::from(CHARLIE)),
				1_000 * UNITS,
			);

			// Charlie can stake now
			assert_ok!(ParachainStaking::join_candidates(
				origin_of(AccountId::from(CHARLIE)),
				1_000 * UNITS,
				0u32,
			),);
			let candidates = ParachainStaking::candidate_pool();
			assert_eq!(
				candidates.0[0],
				Bond {
					owner: AccountId::from(CHARLIE),
					amount: 1_000 * UNITS
				}
			);
		});
}

#[test]
fn reward_block_authors() {
	ExtBuilder::default()
		.with_balances(vec![
			// Alice gets 100 extra tokens for her mapping deposit
			(AccountId::from(ALICE), 2_100 * UNITS),
			(AccountId::from(BOB), 1_000 * UNITS),
		])
		.with_collators(vec![(AccountId::from(ALICE), 1_000 * UNITS)])
		.with_nominations(vec![(
			AccountId::from(BOB),
			AccountId::from(ALICE),
			500 * UNITS,
		)])
		.with_mappings(vec![(
			NimbusId::from_slice(&ALICE_NIMBUS),
			AccountId::from(ALICE),
		)])
		.build()
		.execute_with(|| {
			set_parachain_inherent_data();
			for x in 2..599 {
				set_author(NimbusId::from_slice(&ALICE_NIMBUS));
				run_to_block(x);
			}
			// no rewards doled out yet
			assert_eq!(
				Balances::free_balance(AccountId::from(ALICE)),
				1_000 * UNITS,
			);
			assert_eq!(Balances::free_balance(AccountId::from(BOB)), 500 * UNITS,);
			set_author(NimbusId::from_slice(&ALICE_NIMBUS));
			run_to_block(600);
			// rewards minted and distributed
			assert_eq!(
				Balances::free_balance(AccountId::from(ALICE)),
				1113666666584000000000,
			);
			assert_eq!(
				Balances::free_balance(AccountId::from(BOB)),
				541333333292000000000,
			);
		});
}

#[test]
fn reward_block_authors_with_parachain_bond_reserved() {
	ExtBuilder::default()
		.with_balances(vec![
			// Alice gets 100 extra tokens for her mapping deposit
			(AccountId::from(ALICE), 2_100 * UNITS),
			(AccountId::from(BOB), 1_000 * UNITS),
			(AccountId::from(CHARLIE), UNITS),
		])
		.with_collators(vec![(AccountId::from(ALICE), 1_000 * UNITS)])
		.with_nominations(vec![(
			AccountId::from(BOB),
			AccountId::from(ALICE),
			500 * UNITS,
		)])
		.with_mappings(vec![(
			NimbusId::from_slice(&ALICE_NIMBUS),
			AccountId::from(ALICE),
		)])
		.build()
		.execute_with(|| {
			set_parachain_inherent_data();
			assert_ok!(ParachainStaking::set_parachain_bond_account(
				root_origin(),
				AccountId::from(CHARLIE),
			),);
			for x in 2..599 {
				set_author(NimbusId::from_slice(&ALICE_NIMBUS));
				run_to_block(x);
			}
			// no rewards doled out yet
			assert_eq!(
				Balances::free_balance(AccountId::from(ALICE)),
				1_000 * UNITS,
			);
			assert_eq!(Balances::free_balance(AccountId::from(BOB)), 500 * UNITS,);
			assert_eq!(Balances::free_balance(AccountId::from(CHARLIE)), UNITS,);
			set_author(NimbusId::from_slice(&ALICE_NIMBUS));
			run_to_block(600);
			// rewards minted and distributed
			assert_eq!(
				Balances::free_balance(AccountId::from(ALICE)),
				1079592333275448000000,
			);
			assert_eq!(
				Balances::free_balance(AccountId::from(BOB)),
				528942666637724000000,
			);
			// 30% reserved for parachain bond
			assert_eq!(
				Balances::free_balance(AccountId::from(CHARLIE)),
				47515000000000000000,
			);
		});
}

#[test]
fn initialize_crowdloan_addresses_with_batch_and_pay() {
	ExtBuilder::default()
		.with_balances(vec![
			(AccountId::from(ALICE), 2_000 * UNITS),
			(AccountId::from(BOB), 1_000 * UNITS),
		])
		.with_collators(vec![(AccountId::from(ALICE), 1_000 * UNITS)])
		.with_mappings(vec![(
			NimbusId::from_slice(&ALICE_NIMBUS),
			AccountId::from(ALICE),
		)])
		.with_crowdloan_fund(3_000_000 * UNITS)
		.build()
		.execute_with(|| {
			// set parachain inherent data
			set_parachain_inherent_data();
			set_author(NimbusId::from_slice(&ALICE_NIMBUS));
			for x in 1..3 {
				run_to_block(x);
			}
			// Batch calls always succeed. We just need to check the inner event
			assert_ok!(
				Call::Utility(pallet_utility::Call::<Runtime>::batch_all(vec![
					Call::CrowdloanRewards(
						pallet_crowdloan_rewards::Call::<Runtime>::initialize_reward_vec(
							vec![(
								[4u8; 32].into(),
								Some(AccountId::from(CHARLIE)),
								1_500_000 * UNITS
							)],
							0,
							2
						)
					),
					Call::CrowdloanRewards(
						pallet_crowdloan_rewards::Call::<Runtime>::initialize_reward_vec(
							vec![(
								[5u8; 32].into(),
								Some(AccountId::from(DAVE)),
								1_500_000 * UNITS
							)],
							1,
							2
						)
					)
				]))
				.dispatch(root_origin())
			);
			// 30 percent initial payout
			assert_eq!(
				Balances::balance(&AccountId::from(CHARLIE)),
				450_000 * UNITS
			);
			// 30 percent initial payout
			assert_eq!(Balances::balance(&AccountId::from(DAVE)), 450_000 * UNITS);
			let expected = Event::Utility(pallet_utility::Event::BatchCompleted);
			assert_eq!(last_event(), expected);
			// This one should fail, as we already filled our data
			assert_ok!(Call::Utility(pallet_utility::Call::<Runtime>::batch(vec![
				Call::CrowdloanRewards(
					pallet_crowdloan_rewards::Call::<Runtime>::initialize_reward_vec(
						vec![([4u8; 32].into(), Some(AccountId::from(ALICE)), 432000)],
						0,
						1
					)
				)
			]))
			.dispatch(root_origin()));
			let expected_fail = Event::Utility(pallet_utility::Event::BatchInterrupted(
				0,
				DispatchError::Module {
					index: 20,
					error: 8,
					message: None,
				},
			));
			assert_eq!(last_event(), expected_fail);
			// Claim 1 block.
			assert_ok!(CrowdloanRewards::claim(origin_of(AccountId::from(CHARLIE))));
			assert_ok!(CrowdloanRewards::claim(origin_of(AccountId::from(DAVE))));

			let vesting_period = 4 * WEEKS as u128;
			let per_block = (1_050_000 * UNITS) / vesting_period;

			assert_eq!(
				CrowdloanRewards::accounts_payable(&AccountId::from(CHARLIE))
					.unwrap()
					.claimed_reward,
				(450_000 * UNITS) + per_block
			);
			assert_eq!(
				CrowdloanRewards::accounts_payable(&AccountId::from(DAVE))
					.unwrap()
					.claimed_reward,
				(450_000 * UNITS) + per_block
			);
			// The total claimed reward should be equal to the account balance at this point.
			assert_eq!(
				Balances::balance(&AccountId::from(CHARLIE)),
				(450_000 * UNITS) + per_block
			);
			assert_eq!(
				Balances::balance(&AccountId::from(DAVE)),
				(450_000 * UNITS) + per_block
			);
			assert_noop!(
				CrowdloanRewards::claim(origin_of(AccountId::from(ALICE))),
				pallet_crowdloan_rewards::Error::<Runtime>::NoAssociatedClaim
			);
		});
}

#[test]
fn join_candidates_via_precompile() {
	ExtBuilder::default()
		.with_balances(vec![(AccountId::from(ALICE), 3_000 * UNITS)])
		.build()
		.execute_with(|| {
			let staking_precompile_address = H160::from_low_u64_be(2048);

			// Alice uses the staking precompile to join as a candidate through the EVM
			let gas_limit = 100000u64;
			let gas_price: U256 = 1_000_000_000.into();
			let amount: U256 = (1000 * UNITS).into();
			let candidate_count: U256 = U256::zero();

			// Construct the call data (selector, amount)
			let mut call_data = Vec::<u8>::from([0u8; 68]);
			call_data[0..4].copy_from_slice(&hex_literal::hex!("ad76ed5a"));
			amount.to_big_endian(&mut call_data[4..36]);
			candidate_count.to_big_endian(&mut call_data[36..]);

			assert_ok!(Call::EVM(pallet_evm::Call::<Runtime>::call(
				AccountId::from(ALICE),
				staking_precompile_address,
				call_data,
				U256::zero(), // No value sent in EVM
				gas_limit,
				gas_price,
				None, // Use the next nonce
			))
			.dispatch(<Runtime as frame_system::Config>::Origin::root()));

			// Assert that Alice is now a candidate
			assert!(ParachainStaking::is_candidate(&AccountId::from(ALICE)));

			// Check for the right events.
			let expected_events = vec![
				Event::Balances(pallet_balances::Event::Reserved(
					AccountId::from(ALICE),
					1000 * UNITS,
				)),
				Event::ParachainStaking(parachain_staking::Event::JoinedCollatorCandidates(
					AccountId::from(ALICE),
					1000 * UNITS,
					1000 * UNITS,
				)),
				Event::EVM(pallet_evm::Event::<Runtime>::Executed(
					staking_precompile_address,
				)),
			];

			assert_eq!(
				System::events()
					.into_iter()
					.map(|e| e.event)
					.collect::<Vec<_>>(),
				expected_events
			);
		});
}

#[test]
fn leave_candidates_via_precompile() {
	ExtBuilder::default()
		.with_balances(vec![(AccountId::from(ALICE), 2_000 * UNITS)])
		.with_collators(vec![(AccountId::from(ALICE), 1_000 * UNITS)])
		.build()
		.execute_with(|| {
			let staking_precompile_address = H160::from_low_u64_be(2048);

			// Alice uses the staking precompile to leave_candidates
			let gas_limit = 100000u64;
			let collator_count: U256 = U256::one();
			let gas_price: U256 = 1_000_000_000.into();

			// Construct the leave_candidates call data
			let mut call_data = Vec::<u8>::from([0u8; 36]);
			call_data[0..4].copy_from_slice(&hex_literal::hex!("b7694219"));
			collator_count.to_big_endian(&mut call_data[4..]);

			assert_ok!(Call::EVM(pallet_evm::Call::<Runtime>::call(
				AccountId::from(ALICE),
				staking_precompile_address,
				call_data,
				U256::zero(), // No value sent in EVM
				gas_limit,
				gas_price,
				None, // Use the next nonce
			))
			.dispatch(<Runtime as frame_system::Config>::Origin::root()));

			// Check for the right events.
			let expected_events = vec![
				Event::ParachainStaking(parachain_staking::Event::CollatorScheduledExit(
					1,
					AccountId::from(ALICE),
					3,
				)),
				Event::EVM(pallet_evm::Event::<Runtime>::Executed(
					staking_precompile_address,
				)),
			];

			assert_eq!(
				System::events()
					.into_iter()
					.map(|e| e.event)
					.collect::<Vec<_>>(),
				expected_events
			);
		});
}

#[test]
fn go_online_offline_via_precompile() {
	ExtBuilder::default()
		.with_balances(vec![(AccountId::from(ALICE), 2_000 * UNITS)])
		.with_collators(vec![(AccountId::from(ALICE), 1_000 * UNITS)])
		.build()
		.execute_with(|| {
			// Alice is initialized as a candidate
			assert!(ParachainStaking::is_candidate(&AccountId::from(ALICE)));
			let staking_precompile_address = H160::from_low_u64_be(2048);

			// Alice uses the staking precompile to go offline
			let gas_limit = 100000u64;
			let gas_price: U256 = 1_000_000_000.into();

			// Construct the go_offline call data
			let mut go_offline_call_data = Vec::<u8>::from([0u8; 4]);
			go_offline_call_data[0..4].copy_from_slice(&hex_literal::hex!("767e0450"));

			assert_ok!(Call::EVM(pallet_evm::Call::<Runtime>::call(
				AccountId::from(ALICE),
				staking_precompile_address,
				go_offline_call_data,
				U256::zero(), // No value sent in EVM
				gas_limit,
				gas_price,
				None, // Use the next nonce
			))
			.dispatch(<Runtime as frame_system::Config>::Origin::root()));

			// Check for the right events.
			let mut expected_events = vec![
				Event::ParachainStaking(parachain_staking::Event::CollatorWentOffline(
					1,
					AccountId::from(ALICE),
				)),
				Event::EVM(pallet_evm::Event::<Runtime>::Executed(
					staking_precompile_address,
				)),
			];

			assert_eq!(
				System::events()
					.into_iter()
					.map(|e| e.event)
					.collect::<Vec<_>>(),
				expected_events
			);

			// Construct the go_online call data
			let mut go_online_call_data = Vec::<u8>::from([0u8; 4]);
			go_online_call_data[0..4].copy_from_slice(&hex_literal::hex!("d2f73ceb"));

			assert_ok!(Call::EVM(pallet_evm::Call::<Runtime>::call(
				AccountId::from(ALICE),
				staking_precompile_address,
				go_online_call_data,
				U256::zero(), // No value sent in EVM
				gas_limit,
				gas_price,
				None, // Use the next nonce
			))
			.dispatch(<Runtime as frame_system::Config>::Origin::root()));

			// Check for the right events.
			let mut new_events = vec![
				Event::ParachainStaking(parachain_staking::Event::CollatorBackOnline(
					1,
					AccountId::from(ALICE),
				)),
				Event::EVM(pallet_evm::Event::<Runtime>::Executed(
					staking_precompile_address,
				)),
			];
			expected_events.append(&mut new_events);

			assert_eq!(
				System::events()
					.into_iter()
					.map(|e| e.event)
					.collect::<Vec<_>>(),
				expected_events
			);
		});
}

#[test]
fn candidate_bond_more_less_via_precompile() {
	ExtBuilder::default()
		.with_balances(vec![(AccountId::from(ALICE), 3_000 * UNITS)])
		.with_collators(vec![(AccountId::from(ALICE), 1_000 * UNITS)])
		.build()
		.execute_with(|| {
			// Alice is initialized as a candidate
			assert!(ParachainStaking::is_candidate(&AccountId::from(ALICE)));
			let staking_precompile_address = H160::from_low_u64_be(2048);

			// Alice uses the staking precompile to bond more
			let gas_limit = 100000u64;
			let gas_price: U256 = 1_000_000_000.into();

			// Construct the candidate_bond_more call
			let mut bond_more_call_data = Vec::<u8>::from([0u8; 36]);
			bond_more_call_data[0..4].copy_from_slice(&hex_literal::hex!("c57bd3a8"));
			let bond_more_amount: U256 = (1000 * UNITS).into();
			bond_more_amount.to_big_endian(&mut bond_more_call_data[4..36]);

			assert_ok!(Call::EVM(pallet_evm::Call::<Runtime>::call(
				AccountId::from(ALICE),
				staking_precompile_address,
				bond_more_call_data,
				U256::zero(), // No value sent in EVM
				gas_limit,
				gas_price,
				None, // Use the next nonce
			))
			.dispatch(<Runtime as frame_system::Config>::Origin::root()));

			// Check for the right events.
			let mut expected_events = vec![
				Event::Balances(pallet_balances::Event::Reserved(
					AccountId::from(ALICE),
					1_000 * UNITS,
				)),
				Event::ParachainStaking(parachain_staking::Event::CollatorBondedMore(
					AccountId::from(ALICE),
					1_000 * UNITS,
					2_000 * UNITS,
				)),
				Event::EVM(pallet_evm::Event::<Runtime>::Executed(
					staking_precompile_address,
				)),
			];

			assert_eq!(
				System::events()
					.into_iter()
					.map(|e| e.event)
					.collect::<Vec<_>>(),
				expected_events
			);

			// Construct the go_online call data
			let mut bond_less_call_data = Vec::<u8>::from([0u8; 36]);
			bond_less_call_data[0..4].copy_from_slice(&hex_literal::hex!("289b6ba7"));
			let bond_less_amount: U256 = (500 * UNITS).into();
			bond_less_amount.to_big_endian(&mut bond_less_call_data[4..36]);

			assert_ok!(Call::EVM(pallet_evm::Call::<Runtime>::call(
				AccountId::from(ALICE),
				staking_precompile_address,
				bond_less_call_data,
				U256::zero(), // No value sent in EVM
				gas_limit,
				gas_price,
				None, // Use the next nonce
			))
			.dispatch(<Runtime as frame_system::Config>::Origin::root()));

			// Check for the right events.
			let mut new_events = vec![
				Event::Balances(pallet_balances::Event::Unreserved(
					AccountId::from(ALICE),
					500 * UNITS,
				)),
				Event::ParachainStaking(parachain_staking::Event::CollatorBondedLess(
					AccountId::from(ALICE),
					2_000 * UNITS,
					1_500 * UNITS,
				)),
				Event::EVM(pallet_evm::Event::<Runtime>::Executed(
					staking_precompile_address,
				)),
			];
			expected_events.append(&mut new_events);

			assert_eq!(
				System::events()
					.into_iter()
					.map(|e| e.event)
					.collect::<Vec<_>>(),
				expected_events
			);
		});
}

#[test]
fn nominate_via_precompile() {
	ExtBuilder::default()
		.with_balances(vec![
			(AccountId::from(ALICE), 3_000 * UNITS),
			(AccountId::from(BOB), 3_000 * UNITS),
		])
		.with_collators(vec![(AccountId::from(ALICE), 1_000 * UNITS)])
		.build()
		.execute_with(|| {
			let staking_precompile_address = H160::from_low_u64_be(2048);

			// Bob uses the staking precompile to nominate Alice through the EVM
			let gas_limit = 100000u64;
			let gas_price: U256 = 1_000_000_000.into();
			let nomination_amount: U256 = (1000 * UNITS).into();
			let collator_nominator_count: U256 = U256::zero();
			let nomination_count: U256 = U256::zero();

			// Construct the call data (selector, collator, nomination amount)
			let mut call_data = Vec::<u8>::from([0u8; 132]);
			call_data[0..4].copy_from_slice(&hex_literal::hex!("82f2c8df"));
			call_data[16..36].copy_from_slice(&ALICE);
			nomination_amount.to_big_endian(&mut call_data[36..68]);
			collator_nominator_count.to_big_endian(&mut call_data[68..100]);
			nomination_count.to_big_endian(&mut call_data[100..]);

			assert_ok!(Call::EVM(pallet_evm::Call::<Runtime>::call(
				AccountId::from(BOB),
				staking_precompile_address,
				call_data,
				U256::zero(), // No value sent in EVM
				gas_limit,
				gas_price,
				None, // Use the next nonce
			))
			.dispatch(<Runtime as frame_system::Config>::Origin::root()));

			// Assert that Bob is now nominating Alice
			assert!(ParachainStaking::is_nominator(&AccountId::from(BOB)));

			// Check for the right events.
			let expected_events = vec![
				Event::Balances(pallet_balances::Event::Reserved(
					AccountId::from(BOB),
					1000 * UNITS,
				)),
				Event::ParachainStaking(parachain_staking::Event::Nomination(
					AccountId::from(BOB),
					1000 * UNITS,
					AccountId::from(ALICE),
					NominatorAdded::AddedToTop {
						new_total: 2000 * UNITS,
					},
				)),
				Event::EVM(pallet_evm::Event::<Runtime>::Executed(
					staking_precompile_address,
				)),
			];

			assert_eq!(
				System::events()
					.into_iter()
					.map(|e| e.event)
					.collect::<Vec<_>>(),
				expected_events
			);
		});
}

#[test]
fn leave_nominators_via_precompile() {
	ExtBuilder::default()
		.with_balances(vec![
			(AccountId::from(ALICE), 1_000 * UNITS),
			(AccountId::from(BOB), 1_000 * UNITS),
			(AccountId::from(CHARLIE), 1_500 * UNITS),
		])
		.with_collators(vec![
			(AccountId::from(ALICE), 1_000 * UNITS),
			(AccountId::from(BOB), 1_000 * UNITS),
		])
		.with_nominations(vec![
			(
				AccountId::from(CHARLIE),
				AccountId::from(ALICE),
				500 * UNITS,
			),
			(AccountId::from(CHARLIE), AccountId::from(BOB), 500 * UNITS),
		])
		.build()
		.execute_with(|| {
			// Charlie is initialized as a nominator
			assert!(ParachainStaking::is_nominator(&AccountId::from(CHARLIE)));
			let staking_precompile_address = H160::from_low_u64_be(2048);

			// Charlie uses staking precompile to leave nominator set
			let gas_limit = 100000u64;
			let nomination_count: U256 = 2.into();
			let gas_price: U256 = 1_000_000_000.into();

			// Construct leave_nominators call
			let mut call_data = Vec::<u8>::from([0u8; 36]);
			call_data[0..4].copy_from_slice(&hex_literal::hex!("e8d68a37"));
			nomination_count.to_big_endian(&mut call_data[4..]);

			assert_ok!(Call::EVM(pallet_evm::Call::<Runtime>::call(
				AccountId::from(CHARLIE),
				staking_precompile_address,
				call_data,
				U256::zero(), // No value sent in EVM
				gas_limit,
				gas_price,
				None, // Use the next nonce
			))
			.dispatch(<Runtime as frame_system::Config>::Origin::root()));

			// Charlie is no longer a nominator
			assert!(!ParachainStaking::is_nominator(&AccountId::from(CHARLIE)));

			// Check for the right events.
			let expected_events = vec![
				Event::Balances(pallet_balances::Event::Unreserved(
					AccountId::from(CHARLIE),
					500 * UNITS,
				)),
				Event::ParachainStaking(parachain_staking::Event::NominatorLeftCollator(
					AccountId::from(CHARLIE),
					AccountId::from(ALICE),
					500 * UNITS,
					1_000 * UNITS,
				)),
				Event::Balances(pallet_balances::Event::Unreserved(
					AccountId::from(CHARLIE),
					500 * UNITS,
				)),
				Event::ParachainStaking(parachain_staking::Event::NominatorLeftCollator(
					AccountId::from(CHARLIE),
					AccountId::from(BOB),
					500 * UNITS,
					1_000 * UNITS,
				)),
				Event::ParachainStaking(parachain_staking::Event::NominatorLeft(
					AccountId::from(CHARLIE),
					1_000 * UNITS,
				)),
				Event::EVM(pallet_evm::Event::<Runtime>::Executed(
					staking_precompile_address,
				)),
			];

			assert_eq!(
				System::events()
					.into_iter()
					.map(|e| e.event)
					.collect::<Vec<_>>(),
				expected_events
			);
		});
}

#[test]
fn revoke_nomination_via_precompile() {
	ExtBuilder::default()
		.with_balances(vec![
			(AccountId::from(ALICE), 1_000 * UNITS),
			(AccountId::from(BOB), 1_000 * UNITS),
			(AccountId::from(CHARLIE), 1_500 * UNITS),
		])
		.with_collators(vec![
			(AccountId::from(ALICE), 1_000 * UNITS),
			(AccountId::from(BOB), 1_000 * UNITS),
		])
		.with_nominations(vec![
			(
				AccountId::from(CHARLIE),
				AccountId::from(ALICE),
				500 * UNITS,
			),
			(AccountId::from(CHARLIE), AccountId::from(BOB), 500 * UNITS),
		])
		.build()
		.execute_with(|| {
			// Charlie is initialized as a nominator
			assert!(ParachainStaking::is_nominator(&AccountId::from(CHARLIE)));
			let staking_precompile_address = H160::from_low_u64_be(2048);

			// Charlie uses staking precompile to revoke nomination
			let gas_limit = 100000u64;
			let gas_price: U256 = 1_000_000_000.into();

			// Construct revoke_nomination call
			let mut call_data = Vec::<u8>::from([0u8; 36]);
			call_data[0..4].copy_from_slice(&hex_literal::hex!("4b65c34b"));
			call_data[16..36].copy_from_slice(&ALICE);

			assert_ok!(Call::EVM(pallet_evm::Call::<Runtime>::call(
				AccountId::from(CHARLIE),
				staking_precompile_address,
				call_data,
				U256::zero(), // No value sent in EVM
				gas_limit,
				gas_price,
				None, // Use the next nonce
			))
			.dispatch(<Runtime as frame_system::Config>::Origin::root()));

			// Charlie is still a nominator because only nomination to Alice was revoked
			assert!(ParachainStaking::is_nominator(&AccountId::from(CHARLIE)));

			// Check for the right events.
			let expected_events = vec![
				Event::Balances(pallet_balances::Event::Unreserved(
					AccountId::from(CHARLIE),
					500 * UNITS,
				)),
				Event::ParachainStaking(parachain_staking::Event::NominatorLeftCollator(
					AccountId::from(CHARLIE),
					AccountId::from(ALICE),
					500 * UNITS,
					1_000 * UNITS,
				)),
				Event::EVM(pallet_evm::Event::<Runtime>::Executed(
					staking_precompile_address,
				)),
			];

			assert_eq!(
				System::events()
					.into_iter()
					.map(|e| e.event)
					.collect::<Vec<_>>(),
				expected_events
			);
		});
}

#[test]
fn nominator_bond_more_less_via_precompile() {
	ExtBuilder::default()
		.with_balances(vec![
			(AccountId::from(ALICE), 1_000 * UNITS),
			(AccountId::from(BOB), 1_500 * UNITS),
		])
		.with_collators(vec![(AccountId::from(ALICE), 1_000 * UNITS)])
		.with_nominations(vec![(
			AccountId::from(BOB),
			AccountId::from(ALICE),
			500 * UNITS,
		)])
		.build()
		.execute_with(|| {
			// Bob is initialized as a nominator
			assert!(ParachainStaking::is_nominator(&AccountId::from(BOB)));
			let staking_precompile_address = H160::from_low_u64_be(2048);

			// Alice uses the staking precompile to bond more
			let gas_limit = 100000u64;
			let gas_price: U256 = 1_000_000_000.into();

			// Construct the nominator_bond_more call
			let mut bond_more_call_data = Vec::<u8>::from([0u8; 68]);
			bond_more_call_data[0..4].copy_from_slice(&hex_literal::hex!("971d44c8"));
			bond_more_call_data[16..36].copy_from_slice(&ALICE);
			let bond_more_amount: U256 = (500 * UNITS).into();
			bond_more_amount.to_big_endian(&mut bond_more_call_data[36..68]);

			assert_ok!(Call::EVM(pallet_evm::Call::<Runtime>::call(
				AccountId::from(BOB),
				staking_precompile_address,
				bond_more_call_data,
				U256::zero(), // No value sent in EVM
				gas_limit,
				gas_price,
				None, // Use the next nonce
			))
			.dispatch(<Runtime as frame_system::Config>::Origin::root()));

			// Check for the right events.
			let mut expected_events = vec![
				Event::Balances(pallet_balances::Event::Reserved(
					AccountId::from(BOB),
					500 * UNITS,
				)),
				Event::ParachainStaking(parachain_staking::Event::NominationIncreased(
					AccountId::from(BOB),
					AccountId::from(ALICE),
					1_500 * UNITS,
					true,
					2_000 * UNITS,
				)),
				Event::EVM(pallet_evm::Event::<Runtime>::Executed(
					staking_precompile_address,
				)),
			];

			assert_eq!(
				System::events()
					.into_iter()
					.map(|e| e.event)
					.collect::<Vec<_>>(),
				expected_events
			);

			// Construct the go_online call data
			let mut bond_less_call_data = Vec::<u8>::from([0u8; 68]);
			bond_less_call_data[0..4].copy_from_slice(&hex_literal::hex!("f6a52569"));
			bond_less_call_data[16..36].copy_from_slice(&ALICE);
			let bond_less_amount: U256 = (500 * UNITS).into();
			bond_less_amount.to_big_endian(&mut bond_less_call_data[36..68]);

			assert_ok!(Call::EVM(pallet_evm::Call::<Runtime>::call(
				AccountId::from(BOB),
				staking_precompile_address,
				bond_less_call_data,
				U256::zero(), // No value sent in EVM
				gas_limit,
				gas_price,
				None, // Use the next nonce
			))
			.dispatch(<Runtime as frame_system::Config>::Origin::root()));

			// Check for the right events.
			let mut new_events = vec![
				Event::Balances(pallet_balances::Event::Unreserved(
					AccountId::from(BOB),
					500 * UNITS,
				)),
				Event::ParachainStaking(parachain_staking::Event::NominationDecreased(
					AccountId::from(BOB),
					AccountId::from(ALICE),
					2_000 * UNITS,
					true,
					1_500 * UNITS,
				)),
				Event::EVM(pallet_evm::Event::<Runtime>::Executed(
					staking_precompile_address,
				)),
			];
			expected_events.append(&mut new_events);

			assert_eq!(
				System::events()
					.into_iter()
					.map(|e| e.event)
					.collect::<Vec<_>>(),
				expected_events
			);
		});
}

#[test]
fn is_nominator_via_precompile() {
	ExtBuilder::default()
		.with_balances(vec![
			(AccountId::from(ALICE), 1_000 * UNITS),
			(AccountId::from(BOB), 1_500 * UNITS),
		])
		.with_collators(vec![(AccountId::from(ALICE), 1_000 * UNITS)])
		.with_nominations(vec![(
			AccountId::from(BOB),
			AccountId::from(ALICE),
			500 * UNITS,
		)])
		.build()
		.execute_with(|| {
			// Confirm Bob is initialized as a nominator directly
			assert!(ParachainStaking::is_nominator(&AccountId::from(BOB)));

			let staking_precompile_address = H160::from_low_u64_be(2048);

			// Construct the input data to check if Bob is a nominator
			let mut bob_input_data = Vec::<u8>::from([0u8; 36]);
			bob_input_data[0..4].copy_from_slice(&hex_literal::hex!("8e5080e7"));
			bob_input_data[16..36].copy_from_slice(&BOB);

			// Expected result is an EVM boolean true which is 256 bits long.
			let mut expected_bytes = Vec::from([0u8; 32]);
			expected_bytes[31] = 1;
			let expected_true_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: expected_bytes,
				cost: 1000,
				logs: Default::default(),
			}));

			// Assert precompile reports Bob is a nominator
			assert_eq!(
				Precompiles::execute(
					staking_precompile_address,
					&bob_input_data,
					None, // target_gas is not necessary right now because consumed none now
					&evm_test_context(),
				),
				expected_true_result
			);

			// Construct the input data to check if Charlie is a nominator
			let mut charlie_input_data = Vec::<u8>::from([0u8; 36]);
			charlie_input_data[0..4].copy_from_slice(&hex_literal::hex!("8e5080e7"));
			charlie_input_data[16..36].copy_from_slice(&CHARLIE);

			// Expected result is an EVM boolean false which is 256 bits long.
			expected_bytes = Vec::from([0u8; 32]);
			let expected_false_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: expected_bytes,
				cost: 1000,
				logs: Default::default(),
			}));

			// Assert precompile also reports Charlie as not a nominator
			assert_eq!(
				Precompiles::execute(
					staking_precompile_address,
					&charlie_input_data,
					None,
					&evm_test_context(),
				),
				expected_false_result
			);
		})
}

#[test]
fn is_candidate_via_precompile() {
	ExtBuilder::default()
		.with_balances(vec![(AccountId::from(ALICE), 1_000 * UNITS)])
		.with_collators(vec![(AccountId::from(ALICE), 1_000 * UNITS)])
		.build()
		.execute_with(|| {
			// Confirm Alice is initialized as a candidate directly
			assert!(ParachainStaking::is_candidate(&AccountId::from(ALICE)));

			let staking_precompile_address = H160::from_low_u64_be(2048);

			// Construct the input data to check if Alice is a candidate
			let mut alice_input_data = Vec::<u8>::from([0u8; 36]);
			alice_input_data[0..4].copy_from_slice(&hex_literal::hex!("8545c833"));
			alice_input_data[16..36].copy_from_slice(&ALICE);

			// Expected result is an EVM boolean true which is 256 bits long.
			let mut expected_bytes = Vec::from([0u8; 32]);
			expected_bytes[31] = 1;
			let expected_true_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: expected_bytes,
				cost: 1000,
				logs: Default::default(),
			}));

			// Assert precompile reports Alice is a collator candidate
			assert_eq!(
				Precompiles::execute(
					staking_precompile_address,
					&alice_input_data,
					None, // target_gas is not necessary right now because consumed none now
					&evm_test_context(),
				),
				expected_true_result
			);

			// Construct the input data to check if Bob is a collator candidate
			let mut bob_input_data = Vec::<u8>::from([0u8; 36]);
			bob_input_data[0..4].copy_from_slice(&hex_literal::hex!("8545c833"));
			bob_input_data[16..36].copy_from_slice(&CHARLIE);

			// Expected result is an EVM boolean false which is 256 bits long.
			expected_bytes = Vec::from([0u8; 32]);
			let expected_false_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: expected_bytes,
				cost: 1000,
				logs: Default::default(),
			}));

			// Assert precompile also reports Bob as not a collator candidate
			assert_eq!(
				Precompiles::execute(
					staking_precompile_address,
					&bob_input_data,
					None,
					&evm_test_context(),
				),
				expected_false_result
			);
		})
}

#[test]
fn is_selected_candidate_via_precompile() {
	ExtBuilder::default()
		.with_balances(vec![(AccountId::from(ALICE), 1_000 * UNITS)])
		.with_collators(vec![(AccountId::from(ALICE), 1_000 * UNITS)])
		.build()
		.execute_with(|| {
			// Confirm Alice is selected directly
			assert!(ParachainStaking::is_selected_candidate(&AccountId::from(
				ALICE
			)));

			let staking_precompile_address = H160::from_low_u64_be(2048);

			// Construct the input data to check if Alice is a candidate
			let mut alice_input_data = Vec::<u8>::from([0u8; 36]);
			alice_input_data[0..4].copy_from_slice(&hex_literal::hex!("8f6d27c7"));
			alice_input_data[16..36].copy_from_slice(&ALICE);

			// Expected result is an EVM boolean true which is 256 bits long.
			let mut expected_bytes = Vec::from([0u8; 32]);
			expected_bytes[31] = 1;
			let expected_true_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: expected_bytes,
				cost: 1000,
				logs: Default::default(),
			}));

			// Assert precompile reports Alice is a collator candidate
			assert_eq!(
				Precompiles::execute(
					staking_precompile_address,
					&alice_input_data,
					None, // target_gas is not necessary right now because consumed none now
					&evm_test_context(),
				),
				expected_true_result
			);

			// Construct the input data to check if Bob is a collator candidate
			let mut bob_input_data = Vec::<u8>::from([0u8; 36]);
			bob_input_data[0..4].copy_from_slice(&hex_literal::hex!("8f6d27c7"));
			bob_input_data[16..36].copy_from_slice(&BOB);

			// Expected result is an EVM boolean false which is 256 bits long.
			expected_bytes = Vec::from([0u8; 32]);
			let expected_false_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: expected_bytes,
				cost: 1000,
				logs: Default::default(),
			}));

			// Assert precompile also reports Bob as not a collator candidate
			assert_eq!(
				Precompiles::execute(
					staking_precompile_address,
					&bob_input_data,
					None,
					&evm_test_context(),
				),
				expected_false_result
			);
		})
}

#[test]
fn min_nomination_via_precompile() {
	ExtBuilder::default().build().execute_with(|| {
		let staking_precompile_address = H160::from_low_u64_be(2048);

		let mut get_min_nom = Vec::<u8>::from([0u8; 4]);
		get_min_nom[0..4].copy_from_slice(&hex_literal::hex!("c9f593b2"));

		let min_nomination = 5u128 * UNITS;
		let expected_min: U256 = min_nomination.into();
		let mut buffer = [0u8; 32];
		expected_min.to_big_endian(&mut buffer);
		let expected_result = Some(Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			output: buffer.to_vec(),
			cost: 1000,
			logs: Default::default(),
		}));

		assert_eq!(
			Precompiles::execute(
				staking_precompile_address,
				&get_min_nom,
				None,
				&evm_test_context(),
			),
			expected_result
		);
	});
}

#[test]
fn points_precompile_zero() {
	ExtBuilder::default().build().execute_with(|| {
		let staking_precompile_address = H160::from_low_u64_be(2048);

		// Construct the input data to check points in round one
		// Notice we start in round one, not round zero.
		let mut input_data = Vec::<u8>::from([0u8; 36]);
		input_data[0..4].copy_from_slice(&hex_literal::hex!("9799b4e7"));
		U256::one().to_big_endian(&mut input_data[4..36]);

		// Expected result is zero points because nobody has authored yet.
		let expected_bytes = Vec::from([0u8; 32]);
		let expected_zero_result = Some(Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			output: expected_bytes,
			cost: 1000,
			logs: Default::default(),
		}));

		// Assert that no points have been earned
		assert_eq!(
			Precompiles::execute(
				staking_precompile_address,
				&input_data,
				None,
				&evm_test_context(),
			),
			expected_zero_result
		);
	})
}

#[test]
fn points_precompile_non_zero() {
	ExtBuilder::default()
		.with_balances(vec![(AccountId::from(ALICE), 1_100 * UNITS)])
		.with_collators(vec![(AccountId::from(ALICE), 1_000 * UNITS)])
		.with_mappings(vec![(
			NimbusId::from_slice(&ALICE_NIMBUS),
			AccountId::from(ALICE),
		)])
		.build()
		.execute_with(|| {
			let staking_precompile_address = H160::from_low_u64_be(2048);

			// Alice authors a block
			set_parachain_inherent_data();
			set_author(NimbusId::from_slice(&ALICE_NIMBUS));

			// Construct the input data to check points in round one
			// Notice we start in round one, not round zero.
			let mut input_data = Vec::<u8>::from([0u8; 36]);
			input_data[0..4].copy_from_slice(&hex_literal::hex!("9799b4e7"));
			U256::one().to_big_endian(&mut input_data[4..36]);

			// Expected result is 20 points because each block is one point.
			// Pretty hacky way to make that data structure...
			let mut expected_bytes = Vec::from([0u8; 32]);
			expected_bytes[31] = 20;

			let expected_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: expected_bytes,
				cost: 1000,
				logs: Default::default(),
			}));

			// Assert that 20 points have been earned
			assert_eq!(
				Precompiles::execute(
					staking_precompile_address,
					&input_data,
					None,
					&evm_test_context(),
				),
				expected_result
			);
		})
}

#[test]
fn points_precompile_round_too_big_error() {
	ExtBuilder::default().build().execute_with(|| {
		let staking_precompile_address = H160::from_low_u64_be(2048);

		// We accept the round as a 256-bit integer for easy compatibility with
		// solidity. But the underlying Rust type is `u32`. So here we test that
		// the precompile fails gracefully when too large of a round is passed in.

		// Construct the input data to check points so far this round
		let mut input_data = Vec::<u8>::from([0u8; 36]);
		input_data[0..4].copy_from_slice(&hex_literal::hex!("9799b4e7"));
		U256::max_value().to_big_endian(&mut input_data[4..36]);

		assert_eq!(
			Precompiles::execute(
				staking_precompile_address,
				&input_data,
				None,
				&evm_test_context(),
			),
			Some(Err(ExitError::Other(
				"Round is too large. 32 bit maximum".into()
			)))
		);
	})
}
