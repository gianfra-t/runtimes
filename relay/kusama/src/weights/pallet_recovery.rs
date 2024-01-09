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

//! Autogenerated weights for `pallet_recovery`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-15, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("../kusama-chain-spec.json")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=../kusama-chain-spec.json
// --steps
// 50
// --repeat
// 20
// --pallet=pallet_recovery
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output
// ./kusama-weights/
// --header
// ./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_recovery`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_recovery::WeightInfo for WeightInfo<T> {
	/// Storage: `Recovery::Proxy` (r:1 w:0)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn as_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `215`
		//  Estimated: `3545`
		// Minimum execution time: 8_506_000 picoseconds.
		Weight::from_parts(8_808_000, 0)
			.saturating_add(Weight::from_parts(0, 3545))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Recovery::Proxy` (r:0 w:1)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn set_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_262_000 picoseconds.
		Weight::from_parts(6_641_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:1)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn create_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3816`
		// Minimum execution time: 19_338_000 picoseconds.
		Weight::from_parts(20_082_355, 0)
			.saturating_add(Weight::from_parts(0, 3816))
			// Standard Error: 3_690
			.saturating_add(Weight::from_parts(95_624, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:0)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:1)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	fn initiate_recovery() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `206`
		//  Estimated: `3854`
		// Minimum execution time: 24_986_000 picoseconds.
		Weight::from_parts(25_666_000, 0)
			.saturating_add(Weight::from_parts(0, 3854))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:0)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:1)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn vouch_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `294 + n * (64 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 17_635_000 picoseconds.
		Weight::from_parts(18_406_867, 0)
			.saturating_add(Weight::from_parts(0, 3854))
			// Standard Error: 3_927
			.saturating_add(Weight::from_parts(130_834, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:0)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:0)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::Proxy` (r:1 w:1)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn claim_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `326 + n * (64 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 21_488_000 picoseconds.
		Weight::from_parts(22_303_888, 0)
			.saturating_add(Weight::from_parts(0, 3854))
			// Standard Error: 5_135
			.saturating_add(Weight::from_parts(85_935, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:1)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn close_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `447 + n * (32 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 29_743_000 picoseconds.
		Weight::from_parts(30_806_409, 0)
			.saturating_add(Weight::from_parts(0, 3854))
			// Standard Error: 5_502
			.saturating_add(Weight::from_parts(95_956, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:0)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::Recoverable` (r:1 w:1)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn remove_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `204 + n * (32 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 26_521_000 picoseconds.
		Weight::from_parts(27_559_273, 0)
			.saturating_add(Weight::from_parts(0, 3854))
			// Standard Error: 5_811
			.saturating_add(Weight::from_parts(103_607, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Recovery::Proxy` (r:1 w:1)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn cancel_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `215`
		//  Estimated: `3545`
		// Minimum execution time: 9_804_000 picoseconds.
		Weight::from_parts(10_206_000, 0)
			.saturating_add(Weight::from_parts(0, 3545))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}