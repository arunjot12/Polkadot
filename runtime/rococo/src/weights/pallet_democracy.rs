// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_democracy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_democracy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
	/// Storage: Democracy PublicPropCount (r:1 w:1)
	/// Proof: Democracy PublicPropCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy PublicProps (r:1 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:1 w:0)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(3238), added: 5713, mode: MaxEncodedLen)
	/// Storage: Democracy DepositOf (r:0 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	fn propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4734`
		//  Estimated: `18187`
		// Minimum execution time: 39_492_000 picoseconds.
		Weight::from_parts(39_853_000, 0)
			.saturating_add(Weight::from_parts(0, 18187))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy DepositOf (r:1 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	fn second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3489`
		//  Estimated: `6695`
		// Minimum execution time: 36_683_000 picoseconds.
		Weight::from_parts(37_121_000, 0)
			.saturating_add(Weight::from_parts(0, 6695))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3365`
		//  Estimated: `7260`
		// Minimum execution time: 48_191_000 picoseconds.
		Weight::from_parts(48_936_000, 0)
			.saturating_add(Weight::from_parts(0, 7260))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3387`
		//  Estimated: `7260`
		// Minimum execution time: 52_175_000 picoseconds.
		Weight::from_parts(53_011_000, 0)
			.saturating_add(Weight::from_parts(0, 7260))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy Cancellations (r:1 w:1)
	/// Proof: Democracy Cancellations (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn emergency_cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `299`
		//  Estimated: `3666`
		// Minimum execution time: 26_255_000 picoseconds.
		Weight::from_parts(26_768_000, 0)
			.saturating_add(Weight::from_parts(0, 3666))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy PublicProps (r:1 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Democracy DepositOf (r:1 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:3 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:0 w:1)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(3238), added: 5713, mode: MaxEncodedLen)
	fn blacklist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5843`
		//  Estimated: `18187`
		// Minimum execution time: 96_376_000 picoseconds.
		Weight::from_parts(97_222_000, 0)
			.saturating_add(Weight::from_parts(0, 18187))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:1 w:0)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(3238), added: 5713, mode: MaxEncodedLen)
	fn external_propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3349`
		//  Estimated: `6703`
		// Minimum execution time: 13_815_000 picoseconds.
		Weight::from_parts(14_071_000, 0)
			.saturating_add(Weight::from_parts(0, 6703))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:0 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	fn external_propose_majority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_456_000 picoseconds.
		Weight::from_parts(3_716_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:0 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	fn external_propose_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_610_000 picoseconds.
		Weight::from_parts(3_768_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumCount (r:1 w:1)
	/// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:2)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	fn fast_track() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `3518`
		// Minimum execution time: 27_514_000 picoseconds.
		Weight::from_parts(27_905_000, 0)
			.saturating_add(Weight::from_parts(0, 3518))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:1 w:1)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(3238), added: 5713, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn veto_external() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3452`
		//  Estimated: `6703`
		// Minimum execution time: 31_250_000 picoseconds.
		Weight::from_parts(31_604_000, 0)
			.saturating_add(Weight::from_parts(0, 6703))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy PublicProps (r:1 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Democracy DepositOf (r:1 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn cancel_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5754`
		//  Estimated: `18187`
		// Minimum execution time: 79_757_000 picoseconds.
		Weight::from_parts(83_603_000, 0)
			.saturating_add(Weight::from_parts(0, 18187))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	fn cancel_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `204`
		//  Estimated: `3518`
		// Minimum execution time: 20_034_000 picoseconds.
		Weight::from_parts(20_674_000, 0)
			.saturating_add(Weight::from_parts(0, 3518))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Democracy LowestUnbaked (r:1 w:1)
	/// Proof: Democracy LowestUnbaked (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumCount (r:1 w:0)
	/// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `177 + r * (86 ±0)`
		//  Estimated: `1489 + r * (2676 ±0)`
		// Minimum execution time: 7_053_000 picoseconds.
		Weight::from_parts(10_157_848, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			// Standard Error: 5_462
			.saturating_add(Weight::from_parts(2_710_889, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy LowestUnbaked (r:1 w:1)
	/// Proof: Democracy LowestUnbaked (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumCount (r:1 w:0)
	/// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	/// Proof: Democracy LastTabledWasExternal (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: Democracy NextExternal (r:1 w:0)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy PublicProps (r:1 w:0)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `177 + r * (86 ±0)`
		//  Estimated: `18187 + r * (2676 ±0)`
		// Minimum execution time: 9_585_000 picoseconds.
		Weight::from_parts(13_021_372, 0)
			.saturating_add(Weight::from_parts(0, 18187))
			// Standard Error: 6_031
			.saturating_add(Weight::from_parts(2_707_449, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy VotingOf (r:3 w:3)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn delegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `729 + r * (108 ±0)`
		//  Estimated: `19800 + r * (2676 ±0)`
		// Minimum execution time: 41_109_000 picoseconds.
		Weight::from_parts(46_477_334, 0)
			.saturating_add(Weight::from_parts(0, 19800))
			// Standard Error: 9_372
			.saturating_add(Weight::from_parts(3_815_232, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy VotingOf (r:2 w:2)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn undelegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `426 + r * (108 ±0)`
		//  Estimated: `13530 + r * (2676 ±0)`
		// Minimum execution time: 21_283_000 picoseconds.
		Weight::from_parts(23_372_139, 0)
			.saturating_add(Weight::from_parts(0, 13530))
			// Standard Error: 6_191
			.saturating_add(Weight::from_parts(3_768_585, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy PublicProps (r:0 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	fn clear_public_proposals() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_510_000 picoseconds.
		Weight::from_parts(3_642_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_remove(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `458`
		//  Estimated: `7260`
		// Minimum execution time: 23_647_000 picoseconds.
		Weight::from_parts(36_627_552, 0)
			.saturating_add(Weight::from_parts(0, 7260))
			// Standard Error: 2_937
			.saturating_add(Weight::from_parts(34_132, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_set(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `459 + r * (22 ±0)`
		//  Estimated: `7260`
		// Minimum execution time: 33_932_000 picoseconds.
		Weight::from_parts(35_331_660, 0)
			.saturating_add(Weight::from_parts(0, 7260))
			// Standard Error: 615
			.saturating_add(Weight::from_parts(60_730, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 100]`.
	fn remove_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `661 + r * (26 ±0)`
		//  Estimated: `7260`
		// Minimum execution time: 16_605_000 picoseconds.
		Weight::from_parts(19_057_092, 0)
			.saturating_add(Weight::from_parts(0, 7260))
			// Standard Error: 873
			.saturating_add(Weight::from_parts(68_964, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 100]`.
	fn remove_other_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `661 + r * (26 ±0)`
		//  Estimated: `7260`
		// Minimum execution time: 16_801_000 picoseconds.
		Weight::from_parts(19_166_788, 0)
			.saturating_add(Weight::from_parts(0, 7260))
			// Standard Error: 1_008
			.saturating_add(Weight::from_parts(69_851, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Democracy NextExternal (r:1 w:0)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:0 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn set_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `361`
		//  Estimated: `3556`
		// Minimum execution time: 19_207_000 picoseconds.
		Weight::from_parts(19_693_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:1 w:0)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn clear_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `3518`
		// Minimum execution time: 17_333_000 picoseconds.
		Weight::from_parts(17_555_000, 0)
			.saturating_add(Weight::from_parts(0, 3518))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy PublicProps (r:1 w:0)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:0 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn set_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4893`
		//  Estimated: `18187`
		// Minimum execution time: 33_859_000 picoseconds.
		Weight::from_parts(34_538_000, 0)
			.saturating_add(Weight::from_parts(0, 18187))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy PublicProps (r:1 w:0)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn clear_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4755`
		//  Estimated: `18187`
		// Minimum execution time: 31_155_000 picoseconds.
		Weight::from_parts(31_520_000, 0)
			.saturating_add(Weight::from_parts(0, 18187))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:0 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn set_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `216`
		//  Estimated: `3556`
		// Minimum execution time: 15_924_000 picoseconds.
		Weight::from_parts(16_151_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn clear_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `235`
		//  Estimated: `3666`
		// Minimum execution time: 18_983_000 picoseconds.
		Weight::from_parts(19_280_000, 0)
			.saturating_add(Weight::from_parts(0, 3666))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
