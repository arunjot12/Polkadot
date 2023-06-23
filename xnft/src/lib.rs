#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]
pub use pallet::*;
pub use xcm::prelude::*;
use sp_runtime::traits::Convert;
pub mod weights;
use crate::traits::Parse;
pub use weights::*;
use xcm_executor::traits::WeightBounds;
use crate::TransferKind::{SelfReserveAsset,ToReserve,ToNonReserve};
use orml_traits::GetByKey;
use sp_runtime::traits::Zero;
use frame_support::traits::Contains;
use scale_info::prelude::vec;
use scale_info::prelude::boxed::Box;
use crate::traits::Reserve;
mod traits;

pub struct Transferred<AccountId> {
	pub sender: AccountId,
	pub assets: MultiAssets,
	pub fee: MultiAsset,
	pub dest: MultiLocation,
}
enum TransferKind {
	/// Transfer self reserve asset.
	SelfReserveAsset,
	/// To reserve location.
	ToReserve,
	/// To non-reserve location.
	ToNonReserve,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use xcm::VersionedMultiLocation;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_nfts::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		 type WeightInfo: WeightInfo;
		type CollectionIdConvert: Convert<Self::CollectionId, Option<MultiLocation>>;
		type MultiLocationsFilter: Contains<MultiLocation>;
		type CollectionNativeLocation: Parameter + Member + Default;
		type MaxAssetsForTransfer: Get<usize>;
		type AccountIdToMultiLocation: Convert<Self::AccountId, MultiLocation>;
		type ReserveProvider: Reserve;
		type XcmExecutor: ExecuteXcm<Self::RuntimeCall>;
		type Weigher: WeightBounds<Self::RuntimeCall>;
		type MinXcmFee: GetByKey<MultiLocation, Option<u128>>;
		type UniversalLocation: Get<InteriorMultiLocation>;
		/// Self chain location.
		#[pallet::constant]
		type SelfLocation: Get<MultiLocation>;
		
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		
		TransferredMultiAssets {
			sender: T::AccountId,
			assets: MultiAssets,
			fee: MultiAsset,
			dest: MultiLocation,
		},

		LocationSet {
            collection_id: T::CollectionId,
            location: T::CollectionNativeLocation,
        },
	}

	#[pallet::storage]
    #[pallet::getter(fn locations)]
    /// Native location of an nft.
    pub type AssetLocations<T: Config> = StorageMap<_, Twox64Concat, T::CollectionId, T::CollectionNativeLocation, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn location_assets)]
    /// Local asset for native location.
    pub type LocationAssets<T: Config> =
        StorageMap<_, Blake2_128Concat, T::CollectionNativeLocation, T::CollectionId, OptionQuery>;
		

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		BadVersion,
		NotSupportedMultiLocation,
		NotCrossChainTransferableCurrency,
		TooManyAssetsBeingSent,
		DistinctReserveForAssetAndFee,
		AssetHasNoReserve,
		InvalidDest,
		InvalidAsset,
		AssetIndexNonExistent,
		MinXcmFeeNotDefined,
		FeeNotEnough,
		CannotReanchor,
		NotCrossChainTransfer,
		XcmExecutionFailed,
		NFTLocationAlreadyRegistered,
		UnweighableMessage,
		NftIdNotFOund,
		CannotUpdateLocation
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::do_something())]
		pub fn transfer(
			origin: OriginFor<T>, 
			collection_id : T::CollectionId,
			dest: Box<VersionedMultiLocation>,
			dest_weight_limit: WeightLimit,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let dest: MultiLocation = (*dest).try_into().map_err(|()| Error::<T>::BadVersion)?;
			Self::do_transfer(who, collection_id, dest,dest_weight_limit).map(|_| ())	
		}

			
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::do_something())]
		pub fn register(
			origin: OriginFor<T>,
			 collection_id : T::CollectionId,
			 location: Option<T::CollectionNativeLocation>,
		) -> DispatchResult {

			let who = ensure_signed(origin);

			if let Some(loc) = location {
                //  ensure!(collection_id = pallet_nfts::Collection::<T>::get(&collection_id) , Error::<T>::NftIdNotFOund);
				let collection_details = pallet_nfts::Collection::<T>::get(&collection_id).ok_or(Error::<T>::NftIdNotFOund);
                AssetLocations::<T>::insert(collection_id, &loc);
                LocationAssets::<T>::insert(&loc, collection_id);

				Self::deposit_event(Event::LocationSet {
                    collection_id,
                    location: loc,
                });
            }
	
		


			Ok(())
		}


	}

	impl<T: Config> Pallet<T> {
		fn do_transfer(
			who: T::AccountId,
			collection_id: T::CollectionId,
			dest: MultiLocation,
			dest_weight_limit: WeightLimit,
		) -> Result<Transferred<T::AccountId>, DispatchError> {
			let location: MultiLocation =T::CollectionIdConvert::convert(collection_id).ok_or(Error::<T>::NotCrossChainTransferableCurrency)?;
			let fungibility: Fungibility = Fungibility::Fungible(1u128);
			ensure!(
				T::MultiLocationsFilter::contains(&dest),
				Error::<T>::NotSupportedMultiLocation
			);
			let asset: MultiAsset =(location,fungibility).into();
			Self::do_transfer_multiassets(who, vec![asset.clone()].into(), asset, dest,	dest_weight_limit)			
		}

		fn do_transfer_multiassets(
			who: T::AccountId,
			assets: MultiAssets,
			fee: MultiAsset,
			dest: MultiLocation,
			dest_weight_limit: WeightLimit,
		) -> Result<Transferred<T::AccountId>, DispatchError> {
			ensure!(assets.len() <= T::MaxAssetsForTransfer::get(),	Error::<T>::TooManyAssetsBeingSent);
			ensure!(T::MultiLocationsFilter::contains(&dest),	Error::<T>::NotSupportedMultiLocation	);

			let origin_location = T::AccountIdToMultiLocation::convert(who.clone());
			let mut non_fee_reserve: Option<MultiLocation> = None;
			let asset_len = assets.len();
			for i in 0..asset_len {
				let asset = assets.get(i).ok_or(Error::<T>::AssetIndexNonExistent)?;
				ensure!(
					matches!(asset.fun, Fungibility::Fungible(x) if !x.is_zero()),
					Error::<T>::InvalidAsset
				);
				// `assets` includes fee, the reserve location is decided by non fee asset
				if (fee != *asset && non_fee_reserve.is_none()) || asset_len == 1 {
					non_fee_reserve = T::ReserveProvider::reserve(asset);
				}
				// make sure all non fee assets share the same reserve
				if non_fee_reserve.is_some() {
					ensure!(
						non_fee_reserve == T::ReserveProvider::reserve(asset),
						Error::<T>::DistinctReserveForAssetAndFee
					);
				}
			}

			let fee_reserve = T::ReserveProvider::reserve(&fee);

			if fee_reserve != non_fee_reserve {
				// Current only support `ToReserve` with relay-chain asset as fee. other case
				// like `NonReserve` or `SelfReserve` with relay-chain fee is not support.
				ensure!(non_fee_reserve == dest.chain_part(), Error::<T>::InvalidAsset);

				let reserve_location = non_fee_reserve.ok_or(Error::<T>::AssetHasNoReserve)?;
				let min_xcm_fee = T::MinXcmFee::get(&reserve_location).ok_or(Error::<T>::MinXcmFeeNotDefined)?;

				// min xcm fee should less than user fee
				let fee_to_dest: MultiAsset = (fee.id, min_xcm_fee).into();
				ensure!(fee_to_dest < fee, Error::<T>::FeeNotEnough);

				let mut assets_to_dest = MultiAssets::new();
				for i in 0..asset_len {
					let asset = assets.get(i).ok_or(Error::<T>::AssetIndexNonExistent)?;
					if fee != *asset {
						assets_to_dest.push(asset.clone());
					} else {
						assets_to_dest.push(fee_to_dest.clone());
					}
				}
			
				let mut assets_to_fee_reserve = MultiAssets::new();
				let asset_to_fee_reserve = subtract_fee(&fee, min_xcm_fee);
				assets_to_fee_reserve.push(asset_to_fee_reserve.clone());

				let mut override_recipient = T::SelfLocation::get();
				if override_recipient == MultiLocation::here() {
					let dest_chain_part = dest.chain_part().ok_or(Error::<T>::InvalidDest)?;
					let ancestry = T::UniversalLocation::get();
					let _ = override_recipient
						.reanchor(&dest_chain_part, ancestry)
						.map_err(|_| Error::<T>::CannotReanchor);
				}
				Self::execute_and_send_reserve_kind_xcm(
					origin_location,
					assets_to_fee_reserve,
					asset_to_fee_reserve,
					fee_reserve,
					&dest,
					Some(override_recipient),
					dest_weight_limit.clone(),
					true,
				)?;

				Self::execute_and_send_reserve_kind_xcm(
					origin_location,
					assets_to_dest,
					fee_to_dest,
					non_fee_reserve,
					&dest,
					None,
					dest_weight_limit,
					false,
				)?;

			}
			Self::deposit_event(Event::<T>::TransferredMultiAssets {
				sender: who.clone(),
				assets: assets.clone(),
				fee: fee.clone(),
				dest,
			});

			Ok(Transferred {
				sender: who,
				assets,
				fee,
				dest,
			})	

		
		}
		fn execute_and_send_reserve_kind_xcm(
			origin_location: MultiLocation,
			assets: MultiAssets,
			fee: MultiAsset,
			reserve: Option<MultiLocation>,
			dest: &MultiLocation,
			maybe_recipient_override: Option<MultiLocation>,
			dest_weight_limit: WeightLimit,
			use_teleport: bool,
		) -> DispatchResult {
			let (transfer_kind, dest, reserve, recipient) = Self::transfer_kind(reserve, dest)?;
			let recipient = match maybe_recipient_override {
				Some(recipient) => recipient,
				None => recipient,
			};
			let mut msg = match transfer_kind {
				SelfReserveAsset => Self::transfer_self_reserve_asset(assets, fee, dest, recipient,dest_weight_limit)?,
				ToReserve => Self::transfer_to_reserve(assets, fee, dest, recipient,dest_weight_limit)?,
				ToNonReserve => Self::transfer_to_non_reserve(
					assets,
					fee,
					reserve,
					dest,
					recipient,
					dest_weight_limit,
					use_teleport,
				)?,
			};
			let hash = msg.using_encoded(sp_io::hashing::blake2_256);
	
			let weight = T::Weigher::weight(&mut msg).map_err(|()| Error::<T>::UnweighableMessage)?;
			T::XcmExecutor::execute_xcm_in_credit(origin_location, msg, hash, weight, weight)
				.ensure_complete()
				.map_err(|error| {
					frame_support::log::error!("Failed execute transfer message with {:?}", error);
					Error::<T>::XcmExecutionFailed
				})?;
	
			Ok(())
		}

		fn transfer_self_reserve_asset(
			assets: MultiAssets,
			fee: MultiAsset,
			dest: MultiLocation,
			recipient: MultiLocation,
			dest_weight_limit: WeightLimit,
		) -> Result<Xcm<T::RuntimeCall>, DispatchError> {
			Ok(Xcm(vec![TransferReserveAsset {
				assets: assets.clone(),
				dest,
				xcm: Xcm(vec![
					Self::buy_execution(fee, &dest,dest_weight_limit)?,
					Self::deposit_asset(recipient, assets.len() as u32),
				]),
			}]))
		}

		fn transfer_kind(
			reserve: Option<MultiLocation>,
			dest: &MultiLocation,
		) -> Result<(TransferKind, MultiLocation, MultiLocation, MultiLocation), DispatchError> {
			let (dest, recipient) = Self::ensure_valid_dest(dest)?;

			let self_location = T::SelfLocation::get();
			ensure!(dest != self_location, Error::<T>::NotCrossChainTransfer);
			let reserve = reserve.ok_or(Error::<T>::AssetHasNoReserve)?;
			let transfer_kind = if reserve == self_location {
				SelfReserveAsset
			} else if reserve == dest {
				ToReserve
			} else {
				ToNonReserve
			};
			Ok((transfer_kind, dest, reserve, recipient))
		}

		fn transfer_to_reserve(
			assets: MultiAssets,
			fee: MultiAsset,
			reserve: MultiLocation,
			recipient: MultiLocation,
			dest_weight_limit: WeightLimit,
		) -> Result<Xcm<T::RuntimeCall>, DispatchError> {
			Ok(Xcm(vec![
				WithdrawAsset(assets.clone()),
				InitiateReserveWithdraw {
					assets: All.into(),
					reserve,
					xcm: Xcm(vec![
						Self::buy_execution(fee, &reserve, dest_weight_limit)?,
						Self::deposit_asset(recipient, assets.len() as u32),
					]),
				},
			]))
		}

		fn deposit_asset(recipient: MultiLocation, max_assets: u32) -> Instruction<()> {
			DepositAsset {
				assets: AllCounted(max_assets).into(),
				beneficiary: recipient,
			}
		}

		fn buy_execution(
			asset: MultiAsset,
			at: &MultiLocation,
			weight_limit: WeightLimit,
		) -> Result<Instruction<()>, DispatchError> {
			let ancestry = T::UniversalLocation::get();
			let fees = asset.reanchored(at, ancestry).map_err(|_| Error::<T>::CannotReanchor)?;

			Ok(BuyExecution { fees, weight_limit })
		}

		/// Ensure has the `dest` has chain part and recipient part.
		fn ensure_valid_dest(dest: &MultiLocation) -> Result<(MultiLocation, MultiLocation), DispatchError> {
			if let (Some(dest), Some(recipient)) = (dest.chain_part(), dest.non_chain_part()) {
				Ok((dest, recipient))
			} else {
				Err(Error::<T>::InvalidDest.into())
			}
		}


		fn transfer_to_non_reserve(
			assets: MultiAssets,
			fee: MultiAsset,
			reserve: MultiLocation,
			dest: MultiLocation,
			recipient: MultiLocation,
			dest_weight_limit: WeightLimit,
			use_teleport: bool,
		) -> Result<Xcm<T::RuntimeCall>, DispatchError> {
			let mut reanchored_dest = dest;
			if reserve == MultiLocation::parent() {
				match dest {
					MultiLocation {
						parents,
						interior: X1(Parachain(id)),
					} if parents == 1 => {
						reanchored_dest = Parachain(id).into();
					}
					_ => {}
				}
			}

			let max_assets = assets.len() as u32;
			if !use_teleport {
				Ok(Xcm(vec![
					WithdrawAsset(assets),
					InitiateReserveWithdraw {
						assets: All.into(),
						reserve,
						xcm: Xcm(vec![
							Self::buy_execution(half(&fee), &reserve, dest_weight_limit.clone())?,
							DepositReserveAsset {
								assets: AllCounted(max_assets).into(),
								dest: reanchored_dest,
								xcm: Xcm(vec![
									Self::buy_execution(half(&fee), &dest, dest_weight_limit)?,
									Self::deposit_asset(recipient, max_assets),
								]),
							},
						]),
					},
				]))
			} else {
				Ok(Xcm(vec![
					WithdrawAsset(assets),
					InitiateReserveWithdraw {
						assets: All.into(),
						reserve,
						xcm: Xcm(vec![
							Self::buy_execution(half(&fee), &reserve,dest_weight_limit.clone())?,
							InitiateTeleport {
								assets: All.into(),
								dest: reanchored_dest,
								xcm: Xcm(vec![
									Self::buy_execution(half(&fee), &dest, dest_weight_limit)?,
									Self::deposit_asset(recipient, max_assets),
								]),
							},
						]),
					},
				]))
			}
		}
}
}

/// Returns amount if `asset` is fungible, or zero.
fn fungible_amount(asset: &MultiAsset) -> u128 {
	if let Fungible(amount) = &asset.fun {
		*amount
	} else {
		Zero::zero()
	}
}

fn half(asset: &MultiAsset) -> MultiAsset {
	let half_amount = fungible_amount(asset)
		.checked_div(2)
		.expect("div 2 can't overflow; qed");
	MultiAsset {
		fun: Fungible(half_amount),
		id: asset.id,
	}
}
fn subtract_fee(asset: &MultiAsset, amount: u128) -> MultiAsset {
	let final_amount = fungible_amount(asset).checked_sub(amount).expect("fee too low; qed");
	MultiAsset {
		fun: Fungible(final_amount),
		id: asset.id,
	}
}
