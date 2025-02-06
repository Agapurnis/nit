//! # Nit
//!
//! Nit supplies safe non-binary "bit"-sets abstractions over unsigned integers.
//!
//! ## Features
//! - Zero allocations; all operations can be done in-place.
//! - Runnable at compile-time with a nightly compiler.
//! - Made for `no_std` environments; `std` is used only for optional features[^1].
//! - No external dependencies; speedy compilation and minimal disk space impact.
//! - All `unsafe` code is well-documented and localized to internally marking and skipping preconditions; there is no usage of it relating to memory safety.
//! - Heavily linted and fuzzed for safety and correctness.
//! - Well documented and tested.
//! - No panics; all errors are handled with [`Result`]s.
//!
//! [^1]: The `std` feature may be enabled to enable implementations of [`std::error::Error`] for the error types in this crate,
//!       or the nightly-only [`error-in-core`](#error-in-core) feature can be enabled to use the [`core::error::Error`] trait instead.
//!
//! [`std::error::Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
//! [`core::error::Error`]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html
//!
//! ## How it works
//!
//! You might know how you can use the [modulo operator](https://en.wikipedia.org/wiki/Modulo) to extract the last digit of number:
//! ```
//! assert_eq!(1234 % 10, 4);
//! ```
//!
//! And how you can start dividing by powers of ten to get the other digits:
//! ```
//! assert_eq!(1234 / 10   % 10, 3);
//! assert_eq!(1234 / 100  % 10, 2);
//! assert_eq!(1234 / 1000 % 10, 1);
//! ```
//!
//! This can actually be applied to any base, not just base-10:
//!
//! $$d\_i = \left\lfloor\frac{n}{b^i}\right\rfloor\bmod b$$
//!
//! This can be rewritten to express how an integer would be the composition of such digits (numbered $k$):
//!
//! $$n = \sum_{i=0}^{k} d\_i \cdot b^i$$
//!
//! And to find how many base-$x$ digits ($j$) are are needed to represent fully an integer with $k$ digits in base-$y$:
//!
//! $$c = \left\lceil\log_{x} y^k\right\rceil$$
//!
//! This library acts as a wrapper around this math to make it easier to work with.
//!
//! ## Examples
//!
//! Here's how to store up to five "trits" within a byte.
//!
//! ```
//! use nit::{Trit, prelude::*};
//! let mut data = 0u8;
//! for i in 0..5 {
//!     assert_eq!(data.set_nit(i, Trit::TWO), Ok(Trit::ZERO));
//! }
//! assert_eq!(data, 0b11110010);
//! for i in 0..5 {
//!     assert_eq!(data.get_nit(i),  Some(Trit::TWO));
//! }
//! ```
//!
//! A more complicated example can be found in [`tests/examples/permission-inheritance.rs`](./tests/examples/permission-inheritance.rs).
//!


#![cfg_attr(feature = "nightly", feature(
	derive_const,
	const_mut_refs,
	const_trait_impl,
	effects
))]
#![warn(
	missing_docs,
	clippy::pedantic,
	clippy::nursery,
	clippy::cargo,
	clippy::alloc_instead_of_core,
	clippy::expect_used,
	clippy::unwrap_used,
	clippy::undocumented_unsafe_blocks,
	clippy::missing_docs_in_private_items,
)]
#![allow(clippy::module_name_repetitions)]

#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(all(feature = "nightly", feature = "error-in-core", not(feature = "std")), feature(error_in_core))]
#[cfg(feature = "alloc")] extern crate alloc;

use nit_proc_macros::nit;

/// The base of a number system.
pub mod base;
/// Internal utility macros.
mod internal_macros;
/// Structs representing the placement of a nit.
///
/// For example, in base-10:
/// ```txt
///  3,203
///  │ ││└─ The ones place (10 ** 0); i = 0
///  │ │└── The tens place (10 ** 1); i = 1
///  │ └─── The hundreds place (10 ** 2); i = 2
///  └───── The thousands place (10 ** 3); i = 3
/// ```
pub mod places;
/// The types and constants relevant towards the limitations regarding this crate's functionality and representation of values.
pub mod supported;
/// A trait that can be implemented to retrieve the nits in a number.
pub mod data_container;
/// Utility function and the potential errors that can occur for computing the maximum amount of nits that can be encoded with a number of bits.
pub mod max_nits;
/// Common relevant exports that can be imported with a wildcard.
pub mod prelude;


use internal_macros::{define_empty_error, deriving_const};
use supported::{BaseMaximum, FitsMaximumBits};

define_empty_error!(NitCreationError, "The value is not within the range of 0..BASE.", #[doc = "An error indicating that the value is not within the range of 0..BASE."]);
deriving_const!((PartialEq) for {
	/// A base-`BASE` digit of an integer, which falls in the range of `0..BASE`.
	// TODO: doc better
	#[repr(transparent)]
	#[derive(Debug, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
	pub struct Nit<const BASE: BaseMaximum>(FitsMaximumBits);
});

impl<const BASE: BaseMaximum> Nit<BASE> {
	/// Converts the [`Nit`] into the underlying value.
	///
	/// # Example
	/// ```
	/// use nit::Nit;
	/// let value = Nit::<10>::new(6).unwrap();
	/// assert_eq!(value.into_value(), 6);
	/// ```
	#[must_use]
	pub const fn into_value(self) -> FitsMaximumBits {
		self.0
	}

	/// Returns the underlying value; the digit in the relevant base.
	///
	/// # Example
	/// ```
	/// use nit::Nit;
	/// let value = Nit::<10>::new(6).unwrap();
	/// assert_eq!(value.get_value(), 6);
	/// assert_eq!(value.get_value(), 6);
	/// ```
	#[must_use]
	pub const fn get_value(&self) -> FitsMaximumBits {
		self.0
	}

	/// Returns a new [`Nit`] with the given value, without checking if it is valid.
	///
	/// # Safety
	/// - The value must be within the range of `0..BASE`.
	///
	/// # Undefined Behavior
	/// - Usage of this function with an invalid value is undefined behavior.
	///
	/// # Example
	/// ```
	/// use nit::Nit;
	/// let value = unsafe { Nit::<5>::new_unchecked(8) };
	/// assert_eq!(value.get_value(), 8); // ;c
	/// ```
	#[must_use]
	pub const unsafe fn new_unchecked(value: FitsMaximumBits) -> Self {
		Self(value)
	}

	/// Creates a new [`Nit`] with the given value.
	///
	/// # Returns
	/// A new [`Nit`] with the given value, if it is within the range of `0..BASE`.
	/// Otherwise, it will return an error.
	///
	/// # Errors
	/// - If the value is not within the range of `0..BASE`.
	///
	/// # Example
	/// ```
	/// use nit::Nit;
	/// let value = Nit::<8>::new(5);
	/// assert!(value.is_ok()); // 5 is storable in one digit were it in base-8.
	/// let value = Nit::<5>::new(8);
	/// assert!(value.is_err()); // 8 is not storable in one digit were it in base-5.
	/// ```
	pub const fn new(value: FitsMaximumBits) -> Result<Self, NitCreationError> {
		if value < BASE {
			Ok(Self(value))
		} else {
			Err(NitCreationError)
		}
	}
}
impl<const BASE: BaseMaximum> TryFrom<FitsMaximumBits> for Nit<BASE> {
	type Error = NitCreationError;

	/// Attempts to create and return a new [`Nit`] from a [`FitsMaximumBits`] integer primitive.
	///
	/// # Errors
	/// - If the value is not within the range of `0..BASE`.
	///
	/// # Example
	/// ```
	/// use nit::Nit;
	/// assert!(Nit::<3>::try_from(0).is_ok());
	/// assert!(Nit::<3>::try_from(1).is_ok());
	/// assert!(Nit::<3>::try_from(2).is_ok());
	/// assert!(Nit::<3>::try_from(3).is_err());
	/// ```
	fn try_from(value: FitsMaximumBits) -> Result<Self, Self::Error> {
		Self::new(value)
	}
}

/// A binary [`Nit`] (having a base of `2`).
pub type Bit = Nit<2>;
impl Bit {
	/// A bit of with a value of zero.
	pub const ZERO: Self = Self(0);
	/// A bit of with a value of one.
	pub const ONE: Self = Self(1);
}
/// A tertiary [`Nit`] (having a base of `3`).
pub type Trit = Nit<3>;
impl Trit {
	/// A trit of with a value of zero.
	pub const ZERO: Self = Self(0);
	/// A trit of with a value of one.
	pub const ONE: Self = Self(1);
	/// A trit of with a value of two.
	pub const TWO: Self = Self(2);
}

/// Generates implementations of `From<Nit<BASE>>` for each primitive integer type provided; as all of them are greater than or equal to the maximum of [`FitsMaximumBits`].
macro_rules! impl_numeric_unit_value_wrappers_conversions {
	(($($into: ty),*)) => {
		$(
			impl<const BASE: FitsMaximumBits> From<Nit<BASE>> for $into {
				fn from(value: Nit<BASE>) -> Self {
					#[allow(clippy::cast_lossless)]
					let cast = value.0 as Self;
					cast
				}
			}
		)*
	};
}
impl_numeric_unit_value_wrappers_conversions!((u8, u16, u32, u64, u128));
