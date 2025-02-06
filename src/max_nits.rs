use crate::internal_macros::{deriving_const, impl_error};
use crate::supported::{BaseMaximum, FitsMaximumBits, FitsMaximumBitsAsType, MAXIMUM_SUPPORTED_BITS};
#[cfg(all(test, not(tarpaulin), not(debug_assertions)))] use no_panic::no_panic;

// TODO: These errors should be more clarifying.
deriving_const!((PartialEq) for {
	/// The reason the maximum bit size computation failed.
	#[derive(Debug, Clone, Copy, Eq, Hash)]
	pub enum MaxNitComputationFailure {
		/// The base is less than or equal to 1.
		/// No effective data is representable by this, and it can cause computational errors, so it is not supported.
		BaseTooSmall,
		/// The base are greater than what is currently supported.
		BaseTooLarge,
		/// The bits are zero.
		/// No effective data is representable by this, and it can cause computational errors, so it is not supported.
		BitsTooSmall,
		/// The bits are greater than what is currently supported.
		BitsTooLarge,
		/// The amount of bits can't store enough values to represent at least one base digit.
		BaseExceedsMaxBitValues,
	}
});
impl MaxNitComputationFailure {
	/// Returns the error message as a string.
	#[must_use]
	#[cfg(not(tarpaulin_include))]
	pub const fn get_str(&self) -> &str {
		match self {
			Self::BaseTooSmall => "The base is less than or equal to 1.",
			Self::BaseTooLarge => "The base is greater than what is currently supported.",
			Self::BitsTooSmall => "The bits are zero.",
			Self::BitsTooLarge => "The bits are greater than what is currently supported.",
			Self::BaseExceedsMaxBitValues => "The amount of bits can't store enough values to represent at least one base digit.",
		}
	}
}
impl core::fmt::Display for MaxNitComputationFailure {
	#[cfg(not(tarpaulin_include))]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{}", self.get_str())
	}
}
impl_error!(MaxNitComputationFailure);

/// Computes the maximum amount of base-`BASE` digits that can be stored in a number with `M` bits.
///
/// $${digits} = \left\lceil\log_{base} 2^{bits}\right\rceil$$
///
/// # Errors
/// See: [`MaxNitComputationFailure`]
/// - If the base is less than or equal to 1;
/// - If the base is greater than what is currently supported;
/// - If the bits are zero.
/// - If the bits are greater than what is currently supported.
// Since this is a compile-time function, there isn't any issue in using `u128`, which might otherwise have performance implications.
#[cfg_attr(all(test, not(tarpaulin), not(debug_assertions), feature = "nightly"), no_panic)]
pub const fn compute_max_nits_in_bits<const BASE: BaseMaximum, const BITS: FitsMaximumBits>() -> Result<FitsMaximumBits, MaxNitComputationFailure>  {
	if BITS < 1 { return Err(MaxNitComputationFailure::BitsTooSmall) };
	if BITS > MAXIMUM_SUPPORTED_BITS { return Err(MaxNitComputationFailure::BitsTooLarge) };
	if BASE <= 1 { return Err(MaxNitComputationFailure::BaseTooSmall) };
	if BASE == 2 { return Ok(BITS) };
	if BASE > MAXIMUM_SUPPORTED_BITS { return Err(MaxNitComputationFailure::BaseTooLarge)}
	#[allow(clippy::cast_lossless)]
	let max = if BITS == MAXIMUM_SUPPORTED_BITS { FitsMaximumBitsAsType::MAX } else {
		(1_u128.wrapping_shl(BITS as u32)) - 1
	};
	if max < BASE as u128 - 1 {
		return Err(MaxNitComputationFailure::BaseExceedsMaxBitValues)
	}
	#[allow(clippy::cast_possible_truncation)]
	let log = max.ilog(BASE as FitsMaximumBitsAsType);
	#[allow(clippy::cast_possible_truncation)]
	let log = log as FitsMaximumBits;
	Ok(log)
}
