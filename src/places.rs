use crate::internal_macros::{deriving_const, impl_error};
use crate::supported::{BaseMaximum, FitsMaximumBits};
use crate::max_nits::{compute_max_nits_in_bits, MaxNitComputationFailure};
#[cfg(all(test, not(tarpaulin), not(debug_assertions)))] use no_panic::no_panic;


/// A holder for a value that is functionally equivalent to `BASE.pow(n)`.
///
/// This is the $b^i$ in $d\_i = \left\lfloor\frac{n}{b^i}\right\rfloor\bmod b$.
///
/// *As such, this cannot and must not be equal contain a value equal to zero.*
#[repr(transparent)]
pub struct PlacesShifter<T: Copy, const BASE: BaseMaximum>(T);
impl<T: Copy, const BASE: BaseMaximum> PlacesShifter<T, BASE> {
	/// # Safety
	/// - The value must not be zero.
	/// - The value must be a power of `BASE`.
	pub const unsafe fn new(internal_shift: T) -> Self {
		Self(internal_shift)
	}

	/// Returns the underlying value.
	pub const fn get(&self) -> T {
		self.0
	}
}

deriving_const!((PartialEq) for {
	/// An error that occurred when attempting to create a [`PlacesIndex`].
	#[derive(Debug, Eq, Clone, Copy, Hash)]
	pub enum PlacesIndexCreationError {
		/// The nit limit couldn't be evaluated because either the base or bit count is erroneous; see [`MaxNitComputationFailure`].
		BadNitLimitEvaluation(MaxNitComputationFailure),
		/// The index goes beyond the computed nit capacity.
		OutOfBounds,
	}
});
impl PlacesIndexCreationError {
	/// Returns the error message as a string.
	#[must_use]
	#[cfg(not(tarpaulin_include))]
	pub const fn get_str(&self) -> &str {
		match self {
			Self::BadNitLimitEvaluation(err) => err.get_str(),
			Self::OutOfBounds => "The index goes beyond the computed nit capacity.",
		}
	}
}
#[cfg(not(tarpaulin_include))]
impl core::fmt::Display for PlacesIndexCreationError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{}", self.get_str())
	}
}
impl_error!(PlacesIndexCreationError);

/// The placement of a base-`BASE` digit in a number, starting from the least significant digit (right-hand side).
///
/// This is $i$ in $d\_i = \left\lfloor\frac{n}{b^i}\right\rfloor\bmod b$.
///
/// # Example
/// ```
/// use nit::prelude::*;
/// const VALUE: u8 = 0b1011_1010;
/// type U8BitIndex = PlacesIndex<8, 2>;
/// // Starting from right to left, check extracted base-2 digits are the same with both methods.
/// for n in 0..8 {
///     let idx = U8BitIndex::new(n).unwrap();
///     assert_eq!(
///         VALUE.get_nit_indexed(idx).get_value(),
///         (VALUE >> n) & 1
///     )
/// }
/// ```
// TODO: rename to DigitPosition or something
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq,Eq, Hash, PartialOrd, Ord)]
pub struct PlacesIndex<const TYPE_BIT_WIDTH: FitsMaximumBits, const BASE: BaseMaximum>(FitsMaximumBits);
impl<const TYPE_BIT_WIDTH: FitsMaximumBits, const BASE: BaseMaximum> PlacesIndex<TYPE_BIT_WIDTH, BASE> {
	/// Attempts to create and return a new [`PlacesIndex`] with the given index.
	///
	/// # Returns
	/// A new [`PlacesIndex`] with the given index, if it falls within the valid range for the type.
	/// Otherwise, the function will return an error.
	///
	/// # Errors
	/// - If the base is less than the minimum index (2) for the given base and bits;
	/// - If the index is greater than the maximum index for the given base and bits.
	/// - If the bit width is zero.
	/// - If the bit width is greater than the maximum supported bits.
	///
	/// # Example
	/// ```
	/// use nit::places::PlacesIndex;
	/// assert!(PlacesIndex::<8, 2>::new(7).is_ok());
	/// assert!(PlacesIndex::<8, 2>::new(8).is_err());
	/// ```
	///
	// TODO: needs a better example
	#[cfg_attr(all(test, not(tarpaulin), not(debug_assertions), feature = "nightly"), no_panic)]
	pub const fn new(index: FitsMaximumBits) -> Result<Self, PlacesIndexCreationError> {
		match compute_max_nits_in_bits::<BASE, TYPE_BIT_WIDTH>() {
			Err(err) => {
				Err(PlacesIndexCreationError::BadNitLimitEvaluation(err))
			},
			Ok(max) => {
				// We use '>=' as opposed to '>' since the index is zero-based.
				if index >= max {
					Err(PlacesIndexCreationError::OutOfBounds)
				} else {
					// SAFETY: All precondition checks have been preformed.
					unsafe { Ok(Self::new_unchecked(index)) }
				}
			}
		}
	}

	/// Returns a new [`PlacesIndex`] with the given index, which is assumed to be valid.
	///
	/// # Safety
	/// - The index must be valid.
	/// - The base must be within the range of `2..BASE`.
	/// - The bit width must be within the range of `1..=MAXIMUM_SUPPORTED_BITS`.
	///
	/// # See Also
	/// - [`PlacesIndex::new`]: To create a [`PlacesIndex`] and panic on failure instead of potentially silently encountering undefined behavior.
	///
	/// # Example
	/// ```
	/// use nit::prelude::*;
	/// type BitIndex = PlacesIndex::<1, 2>; // The index into one bit.
	/// assert!(unsafe { BitIndex::new_unchecked(0).get() == 0 });
	/// assert!(unsafe { BitIndex::new_unchecked(1).get() == 1 }); // ;c - technically out of bounds for indexing into one bit
	/// ```
	#[must_use]
	pub const unsafe fn new_unchecked(index: FitsMaximumBits) -> Self {
		Self(index)
	}

	/// Returns the underlying index of the [`PlacesIndex`] instance.
	///
	/// # See Also
	/// - [`PlacesIndex::into`]: To get the index and consume the [`PlacesIndex`] instance.
	///
	/// # Example
	/// ```
	/// use nit::places::PlacesIndex;
	/// type ByteIndex = PlacesIndex::<8, 2>;
	/// for i in 0..8 {
	///    assert_eq!(ByteIndex::new(i).unwrap().get(), i);
	/// }
	/// ```
	#[must_use]
	pub const fn get(&self) -> FitsMaximumBits {
		self.0
	}

	/// Converts this [`PlacesIndex`] into its underlying index.
	///
	/// # See Also
	/// - [`PlacesIndex::get`]: To get the index without consuming the [`PlacesIndex`] instance.
	///
	/// # Example
	/// ```
	/// use nit::places::PlacesIndex;
	/// type ByteIndex = PlacesIndex::<8, 2>;
	/// let index = PlacesIndex::<8, 2>::new(5).unwrap();
	/// assert_eq!(index.into(), 5);
	/// ```
	// TODO: in the example, clarify better the fact we're consuming the index
	#[must_use]
	pub const fn into(self) -> FitsMaximumBits {
		self.0
	}
}
