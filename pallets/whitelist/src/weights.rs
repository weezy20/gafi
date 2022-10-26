
//! Autogenerated weights for `pallet_whitelist`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-13, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `admin`, CPU: `<UNKNOWN>`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/gafi-node
// benchmark
// pallet
// --chain
// dev
// --wasm-execution
// compiled
// --pallet
// pallet_whitelist
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// ./benchmarking/whitelist/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn enable_whitelist(s: u32, ) -> Weight;
	fn apply_whitelist(s: u32, ) -> Weight;
	fn approve_whitelist(s: u32, ) -> Weight;
	fn approve_whitelist_unsigned(s: u32, ) -> Weight;
	fn withdraw_whitelist(s: u32, ) -> Weight;
}

/// Weight functions for `pallet_whitelist`.
pub struct WhitelistWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for WhitelistWeight<T> {
	// Storage: SponsoredPool Pools (r:1 w:0)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: PalletWhitelist WhitelistSource (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn enable_whitelist(_s: u32, ) -> Weight {
		(14_527_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: SponsoredPool Pools (r:1 w:0)
	// Storage: PalletWhitelist Whitelist (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn apply_whitelist(_s: u32, ) -> Weight {
		(5_943_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: SponsoredPool Pools (r:1 w:0)
	// Storage: PalletWhitelist Whitelist (r:1 w:1)
	// Storage: UpfrontPool Services (r:1 w:0)
	// Storage: StakingPool Services (r:1 w:0)
	// Storage: PalletCache DataLeft (r:1 w:0)
	// Storage: PalletCache DataRight (r:1 w:0)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: Pool Tickets (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn approve_whitelist(s: u32, ) -> Weight {
		(26_281_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((1_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: PalletWhitelist Whitelist (r:1 w:1)
	// Storage: UpfrontPool Services (r:1 w:0)
	// Storage: StakingPool Services (r:1 w:0)
	// Storage: PalletCache DataLeft (r:1 w:0)
	// Storage: PalletCache DataRight (r:1 w:0)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: Pool Tickets (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn approve_whitelist_unsigned(s: u32, ) -> Weight {
		(23_199_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}

	// Storage: SponsoredPool Pools (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: PalletWhitelist WhitelistSource (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn withdraw_whitelist(s: u32, ) -> Weight {
		(21_120_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((1_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}

impl WeightInfo for () {
	fn enable_whitelist(_s: u32, ) -> Weight {
		(14_527_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	
	fn apply_whitelist(_s: u32, ) -> Weight {
		(5_943_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	
	fn approve_whitelist(s: u32, ) -> Weight {
		(26_281_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((1_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	
	fn approve_whitelist_unsigned(s: u32, ) -> Weight {
		(23_199_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}

	fn withdraw_whitelist(s: u32, ) -> Weight {
		(21_120_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((1_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
}