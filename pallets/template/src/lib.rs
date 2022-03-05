#![cfg_attr(not(feature = "std"), no_std)]
// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The maximum length Clothes string type.
		#[pallet::constant]
		type MaxLength: Get<u32>;
	}

	// Pallets use events to inform users when important changes are made.
	// Event documentation should end with an array that provides descriptive names for parameters.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClothesCreated(T::AccountId, BoundedVec<u8, T::MaxLength>),
		ClothesRemoved(T::AccountId, BoundedVec<u8, T::MaxLength>),
	}
	#[pallet::error]
	pub enum Error<T> {
		ClothesNotFound,
		ClothesIdAlreadyExists,
		TooLong,
	}
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_clothes)]
	pub(super) type Clothess<T: Config> =
		StorageMap<_, Blake2_128Concat, BoundedVec<u8, T::MaxLength>, ClothesMetadata<T>, OptionQuery>;
	
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct ClothesMetadata<T: Config> {
		pub title: BoundedVec<u8, T::MaxLength>,
		pub description: BoundedVec<u8, T::MaxLength>,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(50_000_000)]
		pub fn create_clothes(
			origin: OriginFor<T>,
			clothes_id: Vec<u8>,
			title: Vec<u8>,
			description: Vec<u8>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let bounded_id: BoundedVec<u8, T::MaxLength> =
				clothes_id.try_into().map_err(|()| Error::<T>::TooLong)?;
			let bounded_title: BoundedVec<u8, T::MaxLength> =
				title.try_into().map_err(|()| Error::<T>::TooLong)?;
			let bounded_description: BoundedVec<u8, T::MaxLength> =
				description.try_into().map_err(|()| Error::<T>::TooLong)?;
			if let Some(_) = <Clothess<T>>::get(&bounded_id) {
				Err(Error::<T>::ClothesIdAlreadyExists)?
			}
			Self::deposit_event(Event::<T>::ClothesCreated(sender, bounded_id.clone()));
			<Clothess<T>>::insert(
				&bounded_id,
				&ClothesMetadata::<T> { title: bounded_title, description: bounded_description },
			);
			Ok(())
		}
		#[pallet::weight(10_000_000)]
		pub fn remove_clothes(origin: OriginFor<T>, clothes_id: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let bounded_id: BoundedVec<u8, T::MaxLength> =
				clothes_id.try_into().map_err(|()| Error::<T>::TooLong)?;
			match <Clothess<T>>::get(&bounded_id) {
				Some(_) => {
					<Clothess<T>>::remove(&bounded_id);
					Self::deposit_event(Event::<T>::ClothesRemoved(sender, bounded_id));
					Ok(())
				},
				None => Err(Error::<T>::ClothesNotFound)?,
			}
		}
	}

	impl<T: Config> MaxEncodedLen for ClothesMetadata<T> {
		fn max_encoded_len() -> usize {
			T::AccountId::max_encoded_len() * 2
		}
	}
}
