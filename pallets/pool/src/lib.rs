// This file is part of Gafi Network.

// Copyright (C) 2021-2022 CryptoViet.
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

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{
	pallet_prelude::*,
	traits::{Currency},
};
use frame_system::pallet_prelude::*;
use gafi_primitives::{
	pool::{StaticPool, Service, TicketType, PlayerTicket, MasterPool, FlexPool},
	constant::{ID},
};
use pallet_timestamp::{self as timestamp};

use crate::weights::WeightInfo;
#[cfg(feature = "std")]
use frame_support::serde::{Deserialize, Serialize};
use scale_info::TypeInfo;
pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{Twox64Concat};

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		
		/// The currency mechanism.
		type Currency: Currency<Self::AccountId>;
		
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// Add upfront pool
		type UpfrontPool: FlexPool<Self::AccountId>;

		/// Add Staking Pool
		type StakingPool: FlexPool<Self::AccountId>;

		/// Add Sponsored Pool
		type SponsoredPool: StaticPool<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Holding the number of tickets to restrict player transaction
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[derive(Eq, PartialEq, Clone, Copy, Encode, Decode, RuntimeDebug, MaxEncodedLen, TypeInfo)]
	pub struct TicketInfo {
		pub ticket_type: TicketType,
		pub tickets: u32,
	}

	impl TicketInfo {
		/// reduce tickets by 1
		pub fn withdraw_ticket(&self) -> Option<Self> {
			if let Some(new_tickets) = self.tickets.checked_sub(1) {
				return Some(TicketInfo {
					tickets: new_tickets,
					ticket_type: self.ticket_type,
				});
			}
			None
		}

		/// renew ticket
		pub  fn renew_ticket(&self, new_remain: u32) -> Self {
			TicketInfo {
				tickets: new_remain,
				ticket_type: self.ticket_type,
			}
		}
	}

	/// Holding all the tickets in the network
	#[pallet::storage]
	pub(super) type Tickets<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, TicketInfo>;

	/// Holding the mark time to check if correct time to charge service fee
	/// The default value is at the time chain launched
	#[pallet::type_value]
	pub fn DefaultMarkTime<T: Config>() -> u128 {
		<timestamp::Pallet<T>>::get().try_into().ok().unwrap()
	}
	#[pallet::storage]
	#[pallet::getter(fn mark_time)]
	pub type MarkTime<T: Config> = StorageValue<_, u128, ValueQuery, DefaultMarkTime<T>>;

	/// Honding the specific period of time to charge service fee
	/// The default value is 1 hours
	#[pallet::type_value]
	pub fn DefaultTimeService() -> u128 {
		// 1 hour
		3_600_000u128
	}
	#[pallet::storage]
	#[pallet::getter(fn time_service)]
	pub type TimeService<T: Config> = StorageValue<_, u128, ValueQuery, DefaultTimeService>;

	/// on_finalize following by steps:
	/// 1. renew tickets
	/// 2. Update new Marktime
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(_block_number: BlockNumberFor<T>) {
			let _now: u128 = <timestamp::Pallet<T>>::get().try_into().ok().unwrap();
			if _now - Self::mark_time() >= Self::time_service() {
				Self::renew_tickets();
				MarkTime::<T>::put(_now);
			}
		}
	}

	//** Genesis Conguration **//
	#[pallet::genesis_config]
	pub struct GenesisConfig {
		pub time_service: u128,
	}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			Self {
				time_service: 3_600_000u128,
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			<TimeService<T>>::put(self.time_service);
			let _now: u128 = <timestamp::Pallet<T>>::get().try_into().ok().unwrap();
			<MarkTime<T>>::put(_now);
		}
	}
		

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Joined { sender: T::AccountId, ticket: TicketType },
		Leaved { sender: T::AccountId, ticket: TicketType },
	}

	#[pallet::error]
	pub enum Error<T> {
		AlreadyJoined,
		NotFoundInPool,
		TicketNotFound,
		ComingSoon,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// join pool
		///
		/// The origin must be Signed
		///
		/// Parameters:
		/// - `ticket`: ticket type
		///
		/// Weight: `O(1)`
		#[pallet::weight(<T as pallet::Config>::WeightInfo::join(50u32, *ticket))]
		pub fn join(origin: OriginFor<T>, ticket: TicketType) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(Tickets::<T>::get(sender.clone()) == None, <Error<T>>::AlreadyJoined);
			let service = Self::verify_ticket(ticket)?;

			match ticket {
				TicketType::Upfront(level) => T::UpfrontPool::join(sender.clone(), level)?,
				TicketType::Staking(level) => T::StakingPool::join(sender.clone(), level)?,
				TicketType::Sponsored(pool_id) => T::SponsoredPool::join(sender.clone(), pool_id)?,
			}

			let ticket_info = TicketInfo {
				ticket_type: ticket,
				tickets: service.tx_limit,
			};

			Tickets::<T>::insert(sender.clone(), ticket_info);
			Self::deposit_event(Event::<T>::Joined { sender, ticket });
			Ok(())
		}

		/// leave pool
		///
		/// The origin must be Signed
		///
		/// Weight: `O(1)`
		#[pallet::weight(<T as pallet::Config>::WeightInfo::leave(50u32))]
		pub fn leave(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			if let Some(ticket) = Tickets::<T>::get(sender.clone()) {
				match ticket.ticket_type {
					TicketType::Upfront(_) => T::UpfrontPool::leave(sender.clone())?,
					TicketType::Staking(_) => T::StakingPool::leave(sender.clone())?,
					TicketType::Sponsored(_) => T::SponsoredPool::leave(sender.clone())?,
				}
				Tickets::<T>::remove(sender.clone());
				Self::deposit_event(Event::<T>::Leaved { sender, ticket: ticket.ticket_type});
				Ok(())
			} else {
				Err(Error::<T>::NotFoundInPool.into())
			}
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn renew_tickets() {
			let _ = Tickets::<T>::iter().for_each(|player| {
				if let Some(ticket_info) = Tickets::<T>::get(player.0.clone()) {
					if let Some(service) = Self::get_service(ticket_info.ticket_type) {
						let new_ticket = ticket_info.renew_ticket(service.tx_limit);
						Tickets::<T>::insert(player.0, new_ticket);
					}
				}
			});
		}

		fn verify_ticket(ticket: TicketType) -> Result<Service, Error<T>> {
			match Self::get_service(ticket) {
				Some(service) => Ok(service),
				None => Err(<Error<T>>::TicketNotFound),
			}
		}
	}

	impl<T: Config> PlayerTicket<T::AccountId> for Pallet<T> {
		fn use_ticket(player: T::AccountId) -> Option<TicketType> {
			if let Some(ticket_info) = Tickets::<T>::get(player.clone()) {
				if let Some(new_ticket_info) = ticket_info.withdraw_ticket() {
					Tickets::<T>::insert(player, new_ticket_info);
					return Some(new_ticket_info.ticket_type);
				}
			}
			None
		}

		fn get_service(ticket: TicketType) -> Option<Service> {
			match ticket {
				TicketType::Upfront(level) => {
					match T::UpfrontPool::get_service(level) {
						Some(service) => Some(service.service),
						None => None
					}
				},
				TicketType::Staking(level) => {
					match T::StakingPool::get_service(level) {
						Some(service) => Some(service.service),
						None => None
					}
				},
				TicketType::Sponsored(pool_id) => {
					match T::SponsoredPool::get_service(pool_id) {
						Some(service) => Some(service.service),
						None => None
					}
				},
			}
		}
	}

	impl<T: Config> MasterPool<T::AccountId> for Pallet<T> {
		fn remove_player(player: &T::AccountId) {
			Tickets::<T>::remove(&player);
		}

		fn get_timeservice() -> u128 {
			TimeService::<T>::get()
		}

		fn get_marktime() -> u128 {
			MarkTime::<T>::get()
		}
	}
}
