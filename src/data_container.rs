use crate::Nit;
use crate::internal_macros::{const_impl, define_const_trait};
use crate::places::{PlacesIndex, PlacesIndexCreationError};
use crate::supported::{BaseMaximum, FitsMaximumBits};
#[cfg(all(test, not(tarpaulin), not(debug_assertions)))] use no_panic::no_panic;

define_const_trait!{
	/// A value that contains numeric data which can be extracted as nits.
	pub NitDataContainer<const TYPE_BIT_WIDTH: FitsMaximumBits> {
		/// Returns the base-`BASE` digit at the `n`th place, falling in the range of `0..BASE`.
		/// Takes in a compile-time-checked [`PlacesIndex`].
		#[must_use]
		fn get_nit_indexed<const BASE: BaseMaximum>(&self, n: PlacesIndex<TYPE_BIT_WIDTH, BASE>) -> Nit<BASE>;

		/// Returns the base-`BASE` digit at the `n`th place, falling in the range of `0..BASE`.
		///
		/// # Safety
		/// It is up to the caller to ensure that the `n`th place is within the valid range for the given base.
		/// See [`PlacesIndex`] for the valid range.
		///
		/// ```
		/// use nit::prelude::*;
		/// let value: u8 = 0;
		///
		/// // Panics in debug builds!
		/// #[cfg(debug_assertions)]
		/// assert!(std::panic::catch_unwind(|| unsafe { value.get_nit_unchecked::<2>(8) }).is_err());
		///
		/// // Undefined behavior in release builds!
		/// #[cfg(not(debug_assertions))]
		/// assert_eq!(unsafe { value.get_nit_unchecked::<2>(8) }, Bit::ZERO);
		/// ```
		#[must_use]
		unsafe fn get_nit_unchecked<const BASE: BaseMaximum>(&self, n: FitsMaximumBits) -> Nit<BASE> {
			// SAFETY: The caller is responsible for ensuring that the index is within the valid range.
			let index = unsafe { PlacesIndex::<TYPE_BIT_WIDTH, BASE>::new_unchecked(n) };
			self.get_nit_indexed(index)
		}

		/// Returns the base-`BASE` digit at the `n`th place, falling in the range of `0..BASE`.
		#[must_use]
		fn get_nit<const BASE: BaseMaximum>(&self, n: FitsMaximumBits) -> Option<Nit<BASE>> {
			match PlacesIndex::<TYPE_BIT_WIDTH, BASE>::new(n) {
				Ok(v) => Some(self.get_nit_indexed(v)),
				Err(_) => None
			}
		}

		/// Sets the base-`BASE` digit at the `n`th place, falling in the range of `0..BASE`.
		/// Returns the previous value at that place.
		/// Takes in a compile-time-checked [`PlacesIndex`].
		fn set_nit_indexed<const BASE: BaseMaximum>(&mut self, n: PlacesIndex<TYPE_BIT_WIDTH, BASE>, value: Nit<BASE>) -> Nit<BASE>;
		/// Sets the base-`BASE` digit at the `n`th place, falling in the range of `0..BASE`.
		/// Returns the previous value at that place.
		///
		/// # Safety
		/// It is up to the caller to ensure that the `n`th place is within the valid range for the given base.
		/// See [`PlacesIndex`] for the valid range.
		unsafe fn set_nit_unchecked<const BASE: BaseMaximum>(&mut self, n: FitsMaximumBits, value: Nit<BASE>) -> Nit<BASE> {
			// SAFETY: The caller is responsible for ensuring that the index is within the valid range.
			let index = unsafe { PlacesIndex::<TYPE_BIT_WIDTH, BASE>::new_unchecked(n) };
			self.set_nit_indexed(index, value)
		}

		/// Sets the base-`BASE` digit at the `n`th place, falling in the range of `0..BASE`.
		/// Returns the previous value at that place, or an error if the index is out of bounds or there was an issue relating to the base or bit count.
		///
		/// # Errors
		/// See: [`PlacesIndexCreationError`]
		///
		/// # Example
		/// ```
		/// use nit::prelude::*;
		/// use nit::Bit;
		/// let mut value: u8 = 0;
		/// assert_eq!(value.set_nit(0, Bit::ONE), Ok(Bit::ZERO));
		/// assert_eq!(value.set_nit(1, Bit::ONE), Ok(Bit::ZERO));
		/// // ...
		/// assert_eq!(value.set_nit(6, Bit::ONE), Ok(Bit::ZERO));
		/// assert_eq!(value.set_nit(7, Bit::ONE), Ok(Bit::ZERO));
		/// assert_eq!(value.set_nit(8, Bit::ONE), Err(PlacesIndexCreationError::OutOfBounds));
		/// ```
		fn set_nit<const BASE: BaseMaximum>(&mut self, n: FitsMaximumBits, value: Nit<BASE>) -> Result<Nit<BASE>, PlacesIndexCreationError> {
			match PlacesIndex::<TYPE_BIT_WIDTH, BASE>::new(n) {
				Ok(v) => Ok(self.set_nit_indexed(v, value)),
				Err(e) => Err(e)
			}
		}
	}
}
/// Generates an implementation of the [`NitDataContainer`] trait for each primitive integer type provided.
macro_rules! impl_numeric_data_container {
	($($type: ty),*) => {
		$(
			const_impl!(NitDataContainer<{ #[allow(clippy::cast_possible_truncation)] { <$type>::BITS as FitsMaximumBits } }> | $type {
				#[cfg_attr(all(test, not(tarpaulin), not(debug_assertions)), no_panic)]
				fn get_nit_indexed<const BASE: BaseMaximum>(&self, n: PlacesIndex<{ #[allow(clippy::cast_possible_truncation)] { <$type>::BITS as FitsMaximumBits } }, { BASE }>) -> Nit<{ BASE }> {
					use crate::base::Base;
					let shifter = <$type>::get_places_shifter(n).get();
					#[allow(clippy::cast_lossless)]
					let modulator = BASE as $type;
					// SAFETY: A shifter is required to be non-zero in its preconditions.
					//         It is meant to functionally equivalent to `BASE.pow(n)`, which is guaranteed to be non-zero.
					//         This fact is further gated behind an `unsafe` constructor, so I think it's fair to make this assumption.
					#[cfg(not(debug_assertions))] unsafe { if shifter == 0 { core::hint::unreachable_unchecked(); } }
					let digit = ((*self / shifter) % modulator);
					#[allow(clippy::cast_possible_truncation)]
					// SAFETY: The value will be always within the range of `0..BASE` because of the modulo operation.
					unsafe { #[allow(clippy::cast_possible_truncation)] let digit = digit as FitsMaximumBits; Nit::new_unchecked(digit) }
				}

				fn set_nit_indexed<const BASE: BaseMaximum>(&mut self, n: PlacesIndex<{ #[allow(clippy::cast_possible_truncation)] { <$type>::BITS as FitsMaximumBits } }, { BASE }>, value: Nit<{ BASE }>) -> Nit<{ BASE }> {
					use crate::base::Base;
					let shifter = <$type>::get_places_shifter(n).get();
					#[allow(clippy::cast_lossless)]
					let modulator = BASE as $type;
					// SAFETY: A shifter is required to be non-zero in its preconditions.
					//         It is meant to functionally equivalent to `BASE.pow(n)`, which is guaranteed to be non-zero.
					//         This fact is further gated behind an `unsafe` constructor, so I think it's fair to make this assumption.
					#[cfg(not(debug_assertions))] unsafe { if shifter == 0 { core::hint::unreachable_unchecked(); } }
					let digit = ((*self / shifter) % modulator);
					// TODO: Document why we overflow?
					let diff = (value.get_value() as $type).overflowing_sub(digit).0;
					// > Assuming that we're not indexing out of bounds, a normal multiplication would overflow on one condition: a set from 0 to B-1 (the stored max value of the nit) on the last nit index.
					// > In that case, we would reach an `adjust` of $T + 1, which should be semantically equivalent to an `adjust` of 0 / no adjust, so it's the correct behavior to preform an overflow.
					// I wrote that earlier but I'm nto sure if that's totally correct anymore.
					let adjust = diff.overflowing_mul(shifter).0;
					// TODO: Document why we can safely add the adjust
					*self = self.overflowing_add(adjust).0;
					// SAFETY: The value will be always within the range of `0..BASE` because of the modulo operation.
					unsafe { #[allow(clippy::cast_possible_truncation)] let digit = digit as FitsMaximumBits; Nit::new_unchecked(digit) }
				}
			});
		)*
	};
}
impl_numeric_data_container!(u8, u16, u32, u64, u128);
