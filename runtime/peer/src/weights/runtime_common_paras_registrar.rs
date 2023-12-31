// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of peer.

// peer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// peer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with peer.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `runtime_common::paras_registrar`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("peer-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/peer
// benchmark
// pallet
// --chain=peer-dev
// --steps=50
// --repeat=20
// --pallet=runtime_common::paras_registrar
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/peer/src/weights/runtime_common_paras_registrar.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_common::paras_registrar`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_common::paras_registrar::WeightInfo for WeightInfo<T> {
	// Storage: Registrar NextFreeParaId (r:1 w:1)
	// Storage: Registrar Paras (r:1 w:1)
	// Storage: Paras ParaLifecycles (r:1 w:0)
	fn reserve() -> Weight {
		// Minimum execution time: 33_118 nanoseconds.
		Weight::from_ref_time(33_824_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Registrar Paras (r:1 w:1)
	// Storage: Paras ParaLifecycles (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Paras PvfActiveVoteMap (r:1 w:0)
	// Storage: Paras CodeByHash (r:1 w:1)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras ActionsQueue (r:1 w:1)
	// Storage: Paras CodeByHashRefs (r:1 w:1)
	// Storage: Paras CurrentCodeHash (r:0 w:1)
	// Storage: Paras UpcomingParasGenesis (r:0 w:1)
	fn register() -> Weight {
		// Minimum execution time: 7_505_264 nanoseconds.
		Weight::from_ref_time(7_578_949_000)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: Registrar Paras (r:1 w:1)
	// Storage: Paras ParaLifecycles (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Paras PvfActiveVoteMap (r:1 w:0)
	// Storage: Paras CodeByHash (r:1 w:1)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras ActionsQueue (r:1 w:1)
	// Storage: Paras CodeByHashRefs (r:1 w:1)
	// Storage: Paras CurrentCodeHash (r:0 w:1)
	// Storage: Paras UpcomingParasGenesis (r:0 w:1)
	fn force_register() -> Weight {
		// Minimum execution time: 7_519_946 nanoseconds.
		Weight::from_ref_time(7_580_737_000)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: Registrar Paras (r:1 w:1)
	// Storage: Paras ParaLifecycles (r:1 w:1)
	// Storage: Paras FutureCodeHash (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras ActionsQueue (r:1 w:1)
	// Storage: Registrar PendingSwap (r:0 w:1)
	fn deregister() -> Weight {
		// Minimum execution time: 50_019 nanoseconds.
		Weight::from_ref_time(51_477_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Registrar Paras (r:1 w:0)
	// Storage: Paras ParaLifecycles (r:2 w:2)
	// Storage: Registrar PendingSwap (r:1 w:1)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras ActionsQueue (r:1 w:1)
	// Storage: Crowdloan Funds (r:2 w:2)
	// Storage: Slots Leases (r:2 w:2)
	fn swap() -> Weight {
		// Minimum execution time: 45_054 nanoseconds.
		Weight::from_ref_time(45_920_000)
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	// Storage: Paras FutureCodeHash (r:1 w:1)
	// Storage: Paras UpgradeRestrictionSignal (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Paras CurrentCodeHash (r:1 w:0)
	// Storage: Paras UpgradeCooldowns (r:1 w:1)
	// Storage: Paras PvfActiveVoteMap (r:1 w:0)
	// Storage: Paras CodeByHash (r:1 w:1)
	// Storage: Paras UpcomingUpgrades (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: Paras CodeByHashRefs (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:0 w:1)
	/// The range of component `b` is `[1, 3145728]`.
	fn schedule_code_upgrade(b: u32, ) -> Weight {
		// Minimum execution time: 43_970 nanoseconds.
		Weight::from_ref_time(44_368_000)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(2_368).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	// Storage: Paras Heads (r:0 w:1)
	/// The range of component `b` is `[1, 1048576]`.
	fn set_current_head(b: u32, ) -> Weight {
		// Minimum execution time: 14_192 nanoseconds.
		Weight::from_ref_time(14_474_000)
			// Standard Error: 2
			.saturating_add(Weight::from_ref_time(966).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
