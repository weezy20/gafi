
//! Autogenerated weights for `pallet_pool`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-12, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/gafi-node
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_pool
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --json-file=raw.json
// --output
// ./pallets/pool/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use gafi_primitives::pool::TicketType;
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn join(s: u32, ticket: TicketType ) -> Weight;
	fn leave(s: u32, ) -> Weight;
}

/// Weight functions for `pallet_pool`.
pub struct PoolWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PoolWeight<T> {
	// Storage: Pool Tickets (r:1 w:1)
	// Storage: UpfrontPool PlayerCount (r:1 w:1)
	// Storage: UpfrontPool MaxPlayer (r:1 w:0)
	// Storage: UpfrontPool NewPlayers (r:1 w:1)
	// Storage: UpfrontPool Services (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: Balances TotalIssuance (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: UpfrontPool Tickets (r:0 w:1)
	// Storage: StakingPool Services (r:1 w:0)
	// Storage: StakingPool PlayerCount (r:1 w:1)
	// Storage: StakingPool Tickets (r:0 w:1)
	fn join(s: u32, ticket: TicketType) -> Weight {
		let base_r = 8;
		let base_w = 4;
		let mut weight = (48_079_000 as Weight).saturating_mul(s as Weight)
		.saturating_add(T::DbWeight::get().reads(base_r as Weight))
		.saturating_add(T::DbWeight::get().writes(base_w as Weight));

		match ticket {
			// r:4 w:3
   			TicketType::Upfront(_) => {
				   let r = 4;
				   let w = 3;
				   weight = (weight as Weight).saturating_add(T::DbWeight::get().reads(r as Weight))
				   .saturating_add(T::DbWeight::get().writes(w as Weight));
			   },
			// r:2 w:2
    		TicketType::Staking(_) => {
				let r = 2;
				let w = 2;
				weight = (weight as Weight).saturating_add(T::DbWeight::get().reads(r as Weight))
				.saturating_add(T::DbWeight::get().writes(w as Weight));
			},
    		TicketType::Sponsored => {
				let r = 0;
				let w = 0;
				weight = (weight as Weight).saturating_add(T::DbWeight::get().reads(r as Weight))
				.saturating_add(T::DbWeight::get().writes(w as Weight));
			},
		}
		weight
	}
	// Storage: Pool Tickets (r:1 w:1)
	// Storage: UpfrontPool Tickets (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: UpfrontPool Services (r:1 w:0)
	// Storage: UpfrontPool TimeService (r:1 w:0)
	// Storage: UpfrontPool PlayerCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: Balances TotalIssuance (r:1 w:1)
	// Storage: UpfrontPool IngamePlayers (r:1 w:1)
	// Storage: UpfrontPool NewPlayers (r:1 w:1)
	// Storage: StakingPool Tickets (r:1 w:1)
	// Storage: StakingPool PlayerCount (r:1 w:1)
	// Storage: StakingPool Services (r:1 w:0)
	fn leave(s: u32 ) -> Weight {
		(56_579_000 as Weight).saturating_mul(s as Weight)
			.saturating_add(T::DbWeight::get().reads(15 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
}

impl WeightInfo for () {
	fn join(s: u32, ticket: TicketType ) -> Weight {
		let base_r = 8;
		let base_w = 4;
		let mut weight = (48_079_000 as Weight).saturating_mul(s as Weight)
		.saturating_add(RocksDbWeight::get().reads(base_r as Weight))
		.saturating_add(RocksDbWeight::get().writes(base_w as Weight));
		match ticket {
			// r:4 w:3
   			TicketType::Upfront(_) => {
				   let r = 4;
				   let w = 3;
				   weight = (weight as Weight).saturating_add(RocksDbWeight::get().reads(r as Weight))
				   .saturating_add(RocksDbWeight::get().writes(w as Weight));
			   },
			// r:2 w:2
    		TicketType::Staking(_) => {
				let r = 2;
				let w = 2;
				weight = (weight as Weight).saturating_add(RocksDbWeight::get().reads(r as Weight))
				.saturating_add(RocksDbWeight::get().writes(w as Weight));
			},
    		TicketType::Sponsored => {
				let r = 0;
				let w = 0;
				weight = (weight as Weight).saturating_add(RocksDbWeight::get().reads(r as Weight))
				.saturating_add(RocksDbWeight::get().writes(w as Weight));
			},
		}
		weight
	}

	fn leave(s: u32,) -> Weight {
		(56_579_000 as Weight).saturating_mul(s as Weight)
			.saturating_add(RocksDbWeight::get().reads(15 as Weight))
			.saturating_add(RocksDbWeight::get().writes(9 as Weight))
	}
}
