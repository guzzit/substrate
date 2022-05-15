// This file is part of Substrate.

// Copyright (C) 2020-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! society pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{account, benchmarks_instance_pallet};
use frame_system::RawOrigin;
use sp_runtime::traits::Bounded;

use crate::Pallet as Society;

const SEED: u32 = 0;

// Create bounties that are approved for use in `on_initialize`.
// fn create_approved_bounties<T: Config>(n: u32) -> Result<(), &'static str> {
// 	for i in 0..n {
// 		let (caller, _curator, _fee, value, reason) =
// 			setup_bounty::<T>(i, T::MaximumReasonLength::get());
// 		Bounties::<T>::propose_bounty(RawOrigin::Signed(caller).into(), value, reason)?;
// 		let bounty_id = BountyCount::<T>::get() - 1;
// 		Bounties::<T>::approve_bounty(RawOrigin::Root.into(), bounty_id)?;
// 	}
// 	ensure!(BountyApprovals::<T>::get().len() == n as usize, "Not all bounty approved");
// 	Ok(())
// }

// Create the pre-requisite information needed to create a society `bid`.
fn setup_bid<T: Config<I>, I: 'static>(
	u: u32,
) -> (T::AccountId, BalanceOf<T, I>,T::AccountId, BalanceOf<T, I>) {
	let caller = account("caller", u, SEED);
	let value: BalanceOf<T, I> = T::CandidateDeposit::get().saturating_mul(100u32.into());
	let _ = T::Currency::make_free_balance_be(&caller, value);
	let tip = value / 2u32.into();
	let who = account("who", u, SEED);
	let _ = T::Currency::make_free_balance_be(&who, tip / 2u32.into());
	//let who_lookup = T::Lookup::unlookup(who);
	//let reason = vec![0; d as usize];
	(caller, value, who, tip)
}

//why isn't this called?
fn setup_pot_account<T: Config>() {
	let pot_account = Society::<T>::account_id();
	let value = T::Currency::minimum_balance().saturating_mul(1_000_000_000u32.into());
	let _ = T::Currency::make_free_balance_be(&pot_account, value);
}

fn setup_payouts_account<T: Config<I>, I: 'static>() {
	let payouts_account = Society::<T, I>::payouts();
	let value = T::Currency::minimum_balance().saturating_mul(1_000_000_000u32.into());
	let _ = T::Currency::make_free_balance_be(&payouts_account, value);
}

fn assert_last_event<T: Config<I>, I: 'static>(generic_event: <T as Config<I>>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks_instance_pallet! {
	bid {
		let (caller, value, who, tip) = setup_bid::<T, _>(SEED);
	}: _(RawOrigin::Signed(caller), value)
	

	unbid {
		let (caller, value, who, tip) = setup_bid::<T, _>(SEED);
		Society::<T, _>::bid(
			RawOrigin::Signed(caller.clone()).into(),
			value
		)?;

		let pos: u32 = 0;//Bids::<T, _>::get().len() - 1;
	}: _(RawOrigin::Signed(caller), pos)

	vouch {
		let (caller, value, who, tip) = setup_bid::<T, _>(SEED);
		//insert caller into member storage?
		Society::<T, _>::add_member(&caller);
	}:  _(RawOrigin::Signed(caller), who, value, tip)

	unvouch {
		let (caller, value, who, tip) = setup_bid::<T, _>(SEED);
		Society::<T, _>::add_member(&caller);
		Society::<T, _>::vouch(
			RawOrigin::Signed(caller.clone()).into(),
			who,
			value,
			tip
		)?;

		let pos: u32 = 0;//Bids::<T, _>::get().len() - 1;
	}: _(RawOrigin::Signed(caller), pos)

	vote {
		//let mut pot = Pot::<T, _>>::get();

		let (caller, value, who, tip) = setup_bid::<T, _>(SEED);
		Society::<T, _>::bid(
			RawOrigin::Signed(who.clone()).into(),
			value
		)?;

		Society::<T, _>::add_member(&caller);

		//let candidates = Self::take_selected(members.len(), pot);
		let candidates = Bids::<T, _>::get();
		Candidates::<T, _>::put(&candidates);

		let candidate_lookup = T::Lookup::unlookup(who);
		let approve = true;
	}: _(RawOrigin::Signed(caller), candidate_lookup, approve)

	defender_vote{
		let (caller, value, who, tip) = setup_bid::<T, _>(SEED);
		Society::<T, _>::bid(
			RawOrigin::Signed(caller.clone()).into(),
			value
		)?;
		Society::<T, _>::add_member(&caller);

		let member_2: T::AccountId = account("member_2", 0, SEED);
		let member_3: T::AccountId = account("member_3", 0, SEED);

		Society::<T, _>::add_member(&member_2);
		Society::<T, _>::add_member(&member_3);
		Defender::<T, _>::put(&member_2);

		let approve = true;
	}: _(RawOrigin::Signed(caller), approve)

	payout {
		let (caller, value, who, tip) = setup_bid::<T, _>(SEED);
		Society::<T, _>::bid(
			RawOrigin::Signed(caller.clone()).into(),
			value
		)?;
		Society::<T, _>::add_member(&caller);

		setup_payouts_account::<T, _>();
		//Society::<T, _>::on_initialize(T::BlockNumber::zero());

		let members = Members::<T, _>::get();

		let maturity = <frame_system::Pallet<T>>::block_number() +
				Society::<T, _>::lock_duration(members.len() as u32);

		Society::<T, _>::bump_payout(&caller, maturity.clone(), value);

		frame_system::Pallet::<T>::set_block_number(maturity);
	}: _(RawOrigin::Signed(caller))

	found {
		//let (caller, value, who, tip) = setup_bid::<T, _>(SEED);
		let founder: T::AccountId = account("who", 0, SEED);
		//let b = RawOrigin::Signed(caller.clone()).into();
		//let origin = Society::FounderSetOrigin::ensure_origin(b);
		//let origin = account("FounderSetAccount", 1, 1);
		let origin = T::FounderSetOrigin::successful_origin();
		let max_members = 100;
		let rules = b"be cool";
		let a = rules.to_vec();
		let f: T::AccountId = founder.clone();
		Head::<T, _>::kill();

	}: _<T::Origin>(origin,  founder, max_members, a)
	verify {
		assert_last_event::<T, _>(Event::Founded { founder:f }.into())
	}//_(RawOrigin::Signed(origin), founder, max_members, a)

	unfound {
		let (caller, value, who, tip) = setup_bid::<T, _>(SEED);
		//let max_members = 100;
		//let rules = b"be cool";
		let origin = T::FounderSetOrigin::successful_origin();
		let max_members = 100;
		let rules = b"be cool";
		let a = rules.to_vec();
		Head::<T, _>::kill();
		Society::<T, _>::found(
			origin,
			caller.clone(),
			max_members,
			a
		)?;
		//MaxMembers::<T, _>::put(max_members);
		// Society::<T, _>::add_member(&caller);
		// Head::<T, _>::put(&caller);
		// Founder::<T, _>::put(&caller);
		// Rules::<T, _>::put(T::Hashing::hash(rules));
	}: _(RawOrigin::Signed(caller))

	// judge_suspended_member {
	// 	//set up payouts
	// 	let (caller, value, who, tip) = setup_bid::<T, _>(SEED);
	// 	Society::<T, _>::bid(
	// 		RawOrigin::Signed(who.clone()).into(),
	// 		value
	// 	)?;
	// 	Society::<T, _>::add_member(&who);
	// 	setup_payouts_account::<T, _>();
	// 	//Society::<T, _>::on_initialize(T::BlockNumber::zero());

	// 	let members = Members::<T, _>::get();

	// 	let maturity = <frame_system::Pallet<T>>::block_number() +
	// 			Society::<T, _>::lock_duration(members.len() as u32);

	// 	Society::<T, _>::bump_payout(&who, maturity, value);

	// 	//set up strikes
	// 	Strikes::<T, _>::mutate(who.clone(), |s| {
	// 						*s += 1;
	// 						*s
	// 					});
	// 	//set up vouching
	// 	let account: T::AccountId = account("account", 0, SEED);

	// 	Society::<T, _>::vouch(
	// 		RawOrigin::Signed(who.clone()).into(),
	// 		account.clone(),
	// 		value,
	// 		tip
	// 	)?;
	// 	SuspendedMembers::<T, _>::insert(&who, true);


	// 	let forgive = false;
	// }: _(RawOrigin::Root, who, forgive)

	// judge_suspended_candidate {

	// 	setup_payouts_account::<T, _>();
	// 	let judgement = Judgement::Reject;
		
	// 	let (caller, value, who, tip) = setup_bid::<T, _>(SEED);
	// 	Society::<T, _>::bid(
	// 		RawOrigin::Signed(who.clone()).into(),
	// 		value
	// 	)?;

	// 	//let candidates = Self::take_selected(members.len(), pot);
	// 	//get bid and put in sus
	// 	let candidates = Bids::<T, _>::get();
	// 	Candidates::<T, _>::put(&candidates);
	// 	let suspended_candidate = candidates[0].clone();
	// 	SuspendedCandidates::<T, _>::insert(&suspended_candidate.who, (suspended_candidate.value, suspended_candidate.kind));
		
	// }: _(RawOrigin::Root, who, judgement)

	// set_max_members {
	// 	let (caller, value, who, tip) = setup_bid::<T, _>(SEED);

	// 	let max_members = 100;

	// }: _(RawOrigin::Root, max_members)
	

	// close_bounty_proposed {
	// 	setup_pot_account::<T>();
	// 	let (caller, curator, fee, value, reason) = setup_bounty::<T>(0, 0);
	// 	Bounties::<T>::propose_bounty(RawOrigin::Signed(caller).into(), value, reason)?;
	// 	let bounty_id = BountyCount::<T>::get() - 1;
	// }: close_bounty(RawOrigin::Root, bounty_id)

	// close_bounty_active {
	// 	setup_pot_account::<T>();
	// 	let (curator_lookup, bounty_id) = create_bounty::<T>()?;
	// 	Bounties::<T>::on_initialize(T::BlockNumber::zero());
	// 	let bounty_id = BountyCount::<T>::get() - 1;
	// }: close_bounty(RawOrigin::Root, bounty_id)
	// verify {
	// 	assert_last_event::<T>(Event::BountyCanceled { index: bounty_id }.into())
	// }

	// extend_bounty_expiry {
	// 	setup_pot_account::<T>();
	// 	let (curator_lookup, bounty_id) = create_bounty::<T>()?;
	// 	Bounties::<T>::on_initialize(T::BlockNumber::zero());

	// 	let bounty_id = BountyCount::<T>::get() - 1;
	// 	let curator = T::Lookup::lookup(curator_lookup).map_err(<&str>::from)?;
	// }: _(RawOrigin::Signed(curator), bounty_id, Vec::new())
	// verify {
	// 	assert_last_event::<T>(Event::BountyExtended { index: bounty_id }.into())
	// }

	// spend_funds {
	// 	let b in 1 .. 100;
	// 	setup_pot_account::<T>();
	// 	create_approved_bounties::<T>(b)?;

	// 	let mut budget_remaining = BalanceOf::<T>::max_value();
	// 	let mut imbalance = PositiveImbalanceOf::<T>::zero();
	// 	let mut total_weight = Weight::zero();
	// 	let mut missed_any = false;
	// }: {
	// 	<Bounties<T> as pallet_treasury::SpendFunds<T>>::spend_funds(
	// 		&mut budget_remaining,
	// 		&mut imbalance,
	// 		&mut total_weight,
	// 		&mut missed_any,
	// 	);
	// }
	// verify {
	// 	ensure!(budget_remaining < BalanceOf::<T>::max_value(), "Budget not used");
	// 	ensure!(missed_any == false, "Missed some");
	// 	assert_last_event::<T>(Event::BountyBecameActive { index: b - 1 }.into())
	// }

	//impl_benchmark_test_suite!(Bounties, crate::tests::new_test_ext(), crate::tests::Test)
	impl_benchmark_test_suite!(Society, crate::tests::new_test_ext(), crate::tests::Test);

}
