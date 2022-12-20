#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

  #[pallet::error]
	pub enum Error<T> {
		RootNumberNotSet,
    StorageOverflow,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NumberIncremented(T::AccountId),
		NumberDecremented(T::AccountId),
	}

  #[pallet::genesis_config]
  pub struct GenesisConfig {
    pub initial_counter: u32,
  }

  #[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			Self { initial_counter: Default::default() }
		}
	}

  #[pallet::genesis_build]
  impl<T: Config> GenesisBuild<T> for GenesisConfig {
    fn build(&self) {
      <Something<T>>::put(0);
    }
  }

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(100)]
		pub fn increment(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
      let old = <Something<T>>::get().unwrap();
      let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
      <Something<T>>::put(new);
      Self::deposit_event(Event::NumberIncremented(who));
      Ok(())
		} 

    #[pallet::weight(100)]
		pub fn decrement(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
      let old = <Something<T>>::get().unwrap();
      let new = old.checked_sub(1).ok_or(Error::<T>::StorageOverflow)?;
      <Something<T>>::put(new);
      Self::deposit_event(Event::NumberDecremented(who));
      Ok(())
		}
	}
}