#[cfg(all(test, not(tarpaulin), not(debug_assertions)))]
use no_panic::no_panic;

#[allow(clippy::wildcard_imports)]
use crate::internal_macros::*;
use crate::places::{PlacesIndex, PlacesShifter};
use crate::supported::{BaseMaximum, FitsMaximumBits};

define_const_trait!(
	/// The base used within a number system.
	pub Base<T: Copy, const TYPE_BIT_WIDTH: FitsMaximumBits, const BASE: BaseMaximum> {
		/// Returns a [`PlacesShifter`] used to get the `n`th base-`BASE` digit of an integer, starting from the least significant digit (right-hand side).
		///
		/// This should be functionally equivalent to `BASE.pow(n)`.
		#[must_use]
		fn get_places_shifter(n: PlacesIndex<TYPE_BIT_WIDTH, BASE>) -> PlacesShifter<T, BASE>;
	}
);

/// Generates an implementation of the `Base` trait for each primitive integer type provided.
macro_rules! impl_base_variants {
	($($type: ty),*) => {
		$(
			// TODO: specialized const
			const_impl_base!(Base<$type, { #[allow(clippy::cast_possible_truncation)] { <$type>::BITS as FitsMaximumBits } }, BASE> | $type {
				#[cfg_attr(all(test, not(tarpaulin), not(debug_assertions)), no_panic)]
				fn get_places_shifter(n: PlacesIndex<{ #[allow(clippy::cast_possible_truncation)] { <$type>::BITS as FitsMaximumBits } }, BASE>) -> PlacesShifter<$type, BASE> {
					// If the `PlacesIndex` precondition was correctly followed, it won't ever overflow.
					// TODO: Can we optimize based on that fact?
					#[allow(clippy::cast_lossless)]
					let shift = (BASE as $type).pow(n.get() as u32); // u32..?
					// SAFETY:
					//  - Any power of `BASE` is guaranteed to be non-zero.
					//  - It will fit the range based on the `PlacesIndex` precondition.
					unsafe { PlacesShifter::new(shift) }
				}
			});
		)*
	};
}
impl_base_variants!(u8, u16, u32, u64, u128);

// define_const_trait!{
// 	/// A value that contains numeric data which can be extracted as nits.
// 	/// This variant is specialized for a specific base.
// 	pub NitDataContainerBased<const TYPE_BIT_WIDTH: FitsMaximumBits, const BASE: BaseMaximum> {
// 		/// Returns the base-`BASE` digit at the `n`th place, falling in the range of `0..BASE`.
// 		/// Takes in a compile-time-checked [`PlacesIndex`].
// 		#[must_use]
// 		fn get_nit_indexed(&self, n: PlacesIndex<TYPE_BIT_WIDTH, BASE>) -> Nit<BASE>;
// 		/// Returns the base-`BASE` digit at the `n`th place, falling in the range of `0..BASE`.
// 		fn get_nit(&self, n: FitsMaximumBits) -> Option<Nit<BASE>> {
// 			match PlacesIndex::<TYPE_BIT_WIDTH, BASE>::new(n) {
// 				Ok(v) => Some(self.get_nit_indexed(v)),
// 				Err(_) => None
// 			}
// 		}
// 	}
// }

#[cfg(test)]
mod tests {
	#![allow(clippy::missing_docs_in_private_items)]
	#![allow(clippy::unwrap_used)]
	#![allow(clippy::undocumented_unsafe_blocks)]
	#[allow(unused)] use super::*;

	macro_rules! make_binary_equivalency_tests {
		($(($name: ident, $type: ty, $bits: literal)),*) => {
			$(
				define_const_func!(#[test] $name() {
					use crate::data_container::NitDataContainer;
					const fn get_nth_bit(value: $type, n: u8) -> u8 { ((value >> n) & 1) as u8 }
					#[allow(clippy::cast_possible_truncation)]
					const BITS: FitsMaximumBits = <$type>::BITS as FitsMaximumBits;
					const VALUE: $type = $bits;
					let mut i = 0;

					while i < BITS {
						let bit = get_nth_bit(VALUE, i);
						let nit = VALUE.get_nit::<2>(i);
						match nit {
							Some(nit) => assert!(bit == nit.into_value()),
							None => assert!(false, "Failed to get nit!"),
						}
						i += 1;
					}
				});
			)*
		};
	}


	make_binary_equivalency_tests!(
		(test_binary_equivalency_u8, u8, 0b1011_1010),
		(test_binary_equivalency_u16, u16, 0b1011_0000_1101_1111),
		(test_binary_equivalency_u32, u32, 0b1011_1010_1100_0010_1101_1111_0000_1110),
		(test_binary_equivalency_u64, u64, 0b0001_1111_1101_0111_0111_1010_1001_0011_0101_1110_1110_0001_1011_1110_1100_1110),
		(test_binary_equivalency_u128, u128, 0b0010_0011_0011_0000_0000_1011_0111_0010_0001_1011_0001_1100_1111_1111_1000_0100_1011_1100_0001_0000_0111_0101_1011_0001_0001_0110_0000_1111_0011_0010_1000_1101)
	);


	// mfw only const way to extract values of errors in const context is pattern matching
	macro_rules! assert_result {
		($val: expr, Ok($expected: expr)) => {
			match $val {
				Ok(val) => assert!(val == $expected),
				Err(_) => assert!($val.is_ok()) // lol
			}
		};
		($val: expr, Err($expected: expr)) => {
			match $val {
				Ok(_) => assert!($val.is_err()), // lol
				Err(val) => assert!(val == $expected),
			}
		};
	}
	macro_rules! unwrap_result {
		($val: expr) => {
			match $val {
				Ok(val) => val,
				Err(_) => panic!("Expected Ok, got Err")
			}
		};
	}

	mod index_creation {
		use crate::internal_macros::*;
		use crate::prelude::*;

		define_const_func!(#[test] out_of_bounds() {
			assert!(PlacesIndex::<8, 2>::new(7).is_ok());
			assert_result!(PlacesIndex::<8, 2>::new(8), Err(PlacesIndexCreationError::OutOfBounds));
			assert_result!(PlacesIndex::<8, 2>::new(9), Err(PlacesIndexCreationError::OutOfBounds));
		});

		define_const_func!(#[test] too_little_bits() {
			assert!(PlacesIndex::<1, 2>::new(0).is_ok());
			assert_result!(PlacesIndex::<0, 2>::new(0), Err(PlacesIndexCreationError::BadNitLimitEvaluation(MaxNitComputationFailure::BitsTooSmall)));
		});

		define_const_func!(#[test] too_many_bits() {
			assert!(PlacesIndex::<127, 2>::new(0).is_ok());
			assert!(PlacesIndex::<128, 2>::new(0).is_ok());
			assert_result!(PlacesIndex::<129, 2>::new(0), Err(PlacesIndexCreationError::BadNitLimitEvaluation(MaxNitComputationFailure::BitsTooLarge)));
		});
		define_const_func!(#[test] base_higher_than_bits() {
			assert!(PlacesIndex::<1, 2>::new(0).is_ok());
			assert_result!(PlacesIndex::<1, 3>::new(0), Err(PlacesIndexCreationError::BadNitLimitEvaluation(MaxNitComputationFailure::BaseExceedsMaxBitValues)));
		});

		define_const_func!(#[test] too_large_of_a_base() {
			assert!(PlacesIndex::<128, 127>::new(0).is_ok());
			assert!(PlacesIndex::<128, 128>::new(0).is_ok());
			assert_result!(PlacesIndex::<128, 129>::new(0), Err(PlacesIndexCreationError::BadNitLimitEvaluation(MaxNitComputationFailure::BaseTooLarge)));
		});

		define_const_func!(#[test] too_small_of_a_base() {
			assert!(PlacesIndex::<128, 2>::new(0).is_ok());
			assert_result!(PlacesIndex::<128, 1>::new(0), Err(PlacesIndexCreationError::BadNitLimitEvaluation(MaxNitComputationFailure::BaseTooSmall)));
			assert_result!(PlacesIndex::<128, 0>::new(0), Err(PlacesIndexCreationError::BadNitLimitEvaluation(MaxNitComputationFailure::BaseTooSmall)));
		});
	}

	mod setting {
		use crate::internal_macros::*;
		use crate::prelude::*;

		macro_rules! test_sets {
			($data: ident, $ty: ident, @@ $index: expr, $set_to: expr, $returns: expr) => {
				assert!(unwrap_result!($data.set_nit($index, unwrap_result!($ty::new($set_to)))).get_value() == $returns);
			};
			($data: ident, $ty: ident, @@ $index: expr, $set_to: expr) => {
				assert!($data.set_nit($index, unwrap_result!($ty::new($set_to))).is_ok());
			};
			($data: ident, $ty: ident, [$((idx $idx: expr, val $val: expr$(, ret $ret: expr)?)),*]) => {
				$(
					test_sets!($data, $ty, @@ $idx, $val $(, $ret)?);
				)*
			};
		}

		define_const_func!(#[test] returns_previous_values() {
			use crate::Trit;
			let mut data: u8 = 0;
			// initial zeros
			test_sets!(data, Trit, [
				(idx 0, val 1, ret 0),
				(idx 1, val 2, ret 0),
				(idx 2, val 0, ret 0),
				(idx 3, val 0, ret 0),
				(idx 4, val 2, ret 0)
			]);
			test_sets!(data, Trit, [
				(idx 0, val 2, ret 1),
				(idx 1, val 2, ret 2),
				(idx 2, val 0, ret 0),
				(idx 3, val 1, ret 0),
				(idx 4, val 1, ret 2)
			]);
		});

		define_const_func!(#[test] miscellaneous () {
			use crate::Trit;
			let mut data: u8 = 0;
			test_sets!(data, Trit, [
				(idx 0, val 2, ret 0),
				(idx 1, val 2, ret 0),
				(idx 2, val 2, ret 0),
				(idx 3, val 2, ret 0),
				(idx 4, val 2, ret 0)
			]);
			test_sets!(data, Trit, [
				(idx 0, val 0, ret 2),
				(idx 1, val 0, ret 2),
				(idx 2, val 0, ret 2),
				(idx 3, val 0, ret 2),
				(idx 4, val 0, ret 2)
			]);
			test_sets!(data, Trit, [
				(idx 0, val 2, ret 0),
				(idx 1, val 0, ret 0),
				(idx 2, val 2, ret 0),
				(idx 3, val 0, ret 0),
				(idx 4, val 2, ret 0)
			]);
			test_sets!(data, Trit, [
				(idx 0, val 0, ret 2),
				(idx 1, val 2, ret 0),
				(idx 2, val 0, ret 2),
				(idx 3, val 2, ret 0),
				(idx 4, val 0, ret 2)
			]);
			test_sets!(data, Trit, [
				(idx 0, val 0, ret 0),
				(idx 1, val 0, ret 2),
				(idx 2, val 0, ret 0),
				(idx 3, val 0, ret 2),
				(idx 4, val 0, ret 0)
			]);
			// reverse order
			test_sets!(data, Trit, [
				(idx 4, val 2, ret 0),
				(idx 3, val 2, ret 0),
				(idx 2, val 2, ret 0),
				(idx 1, val 2, ret 0),
				(idx 0, val 2, ret 0)
			]);
			test_sets!(data, Trit, [
				(idx 4, val 0, ret 2),
				(idx 3, val 0, ret 2),
				(idx 2, val 0, ret 2),
				(idx 1, val 0, ret 2),
				(idx 0, val 0, ret 2)
			]);
		});
	}
}
