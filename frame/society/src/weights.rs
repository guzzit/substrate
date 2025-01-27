// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_society
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_society
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/society/src/weights.rs
// --template=.maintain/frame-weight-template.hbs
// --header=HEADER-APACHE2

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_society.
pub trait WeightInfo {
	fn bid() -> Weight;
	fn unbid() -> Weight;
	fn vouch() -> Weight;
	fn unvouch() -> Weight;
	fn vote() -> Weight;
	fn defender_vote() -> Weight;
	fn payout() -> Weight;
	fn found() -> Weight;
	fn unfound() -> Weight;
	fn judge_suspended_member() -> Weight;
	fn judge_suspended_candidate() -> Weight;
	fn set_max_members() -> Weight;
}

/// Weights for pallet_society using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Society SuspendedCandidates (r:1 w:0)
	// Storage: Society SuspendedMembers (r:1 w:0)
	// Storage: Society Bids (r:1 w:1)
	// Storage: Society Candidates (r:1 w:0)
	// Storage: Society Members (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn bid() -> Weight {
		(50_500_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Society Bids (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unbid() -> Weight {
		(41_600_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Society SuspendedCandidates (r:1 w:0)
	// Storage: Society SuspendedMembers (r:1 w:0)
	// Storage: Society Bids (r:1 w:1)
	// Storage: Society Candidates (r:1 w:0)
	// Storage: Society Members (r:1 w:0)
	// Storage: Society Vouching (r:1 w:1)
	fn vouch() -> Weight {
		(42_700_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Society Vouching (r:1 w:1)
	// Storage: Society Bids (r:1 w:1)
	fn unvouch() -> Weight {
		(33_200_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Society Candidates (r:1 w:0)
	// Storage: Society Members (r:1 w:0)
	// Storage: Society Votes (r:0 w:1)
	fn vote() -> Weight {
		(31_200_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Society Members (r:1 w:0)
	// Storage: Society DefenderVotes (r:0 w:1)
	fn defender_vote() -> Weight {
		(27_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Society Members (r:1 w:0)
	// Storage: Society Payouts (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn payout() -> Weight {
		(54_900_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Society Head (r:1 w:1)
	// Storage: Society Members (r:1 w:1)
	// Storage: Society Founder (r:0 w:1)
	// Storage: Society Rules (r:0 w:1)
	// Storage: Society MaxMembers (r:0 w:1)
	fn found() -> Weight {
		(31_600_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Society Founder (r:1 w:1)
	// Storage: Society Head (r:1 w:1)
	// Storage: Society Candidates (r:0 w:1)
	// Storage: Society Rules (r:0 w:1)
	// Storage: Society Members (r:0 w:1)
	fn unfound() -> Weight {
		(35_600_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Society Founder (r:1 w:0)
	// Storage: Society SuspendedMembers (r:1 w:1)
	// Storage: Society Vouching (r:1 w:1)
	// Storage: Society Bids (r:1 w:1)
	// Storage: Society Strikes (r:0 w:1)
	// Storage: Society Payouts (r:0 w:1)
	fn judge_suspended_member() -> Weight {
		(50_900_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Society Founder (r:1 w:0)
	// Storage: Society SuspendedCandidates (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	fn judge_suspended_candidate() -> Weight {
		(21_600_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Society MaxMembers (r:0 w:1)
	fn set_max_members() -> Weight {
		(17_100_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Society SuspendedCandidates (r:1 w:0)
	// Storage: Society SuspendedMembers (r:1 w:0)
	// Storage: Society Bids (r:1 w:1)
	// Storage: Society Candidates (r:1 w:0)
	// Storage: Society Members (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn bid() -> Weight {
		(50_500_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Society Bids (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unbid() -> Weight {
		(41_600_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Society SuspendedCandidates (r:1 w:0)
	// Storage: Society SuspendedMembers (r:1 w:0)
	// Storage: Society Bids (r:1 w:1)
	// Storage: Society Candidates (r:1 w:0)
	// Storage: Society Members (r:1 w:0)
	// Storage: Society Vouching (r:1 w:1)
	fn vouch() -> Weight {
		(42_700_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Society Vouching (r:1 w:1)
	// Storage: Society Bids (r:1 w:1)
	fn unvouch() -> Weight {
		(33_200_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Society Candidates (r:1 w:0)
	// Storage: Society Members (r:1 w:0)
	// Storage: Society Votes (r:0 w:1)
	fn vote() -> Weight {
		(31_200_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Society Members (r:1 w:0)
	// Storage: Society DefenderVotes (r:0 w:1)
	fn defender_vote() -> Weight {
		(27_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Society Members (r:1 w:0)
	// Storage: Society Payouts (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn payout() -> Weight {
		(54_900_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Society Head (r:1 w:1)
	// Storage: Society Members (r:1 w:1)
	// Storage: Society Founder (r:0 w:1)
	// Storage: Society Rules (r:0 w:1)
	// Storage: Society MaxMembers (r:0 w:1)
	fn found() -> Weight {
		(31_600_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Society Founder (r:1 w:1)
	// Storage: Society Head (r:1 w:1)
	// Storage: Society Candidates (r:0 w:1)
	// Storage: Society Rules (r:0 w:1)
	// Storage: Society Members (r:0 w:1)
	fn unfound() -> Weight {
		(35_600_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Society Founder (r:1 w:0)
	// Storage: Society SuspendedMembers (r:1 w:1)
	// Storage: Society Vouching (r:1 w:1)
	// Storage: Society Bids (r:1 w:1)
	// Storage: Society Strikes (r:0 w:1)
	// Storage: Society Payouts (r:0 w:1)
	fn judge_suspended_member() -> Weight {
		(50_900_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Society Founder (r:1 w:0)
	// Storage: Society SuspendedCandidates (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	fn judge_suspended_candidate() -> Weight {
		(21_600_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Society MaxMembers (r:0 w:1)
	fn set_max_members() -> Weight {
		(17_100_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
