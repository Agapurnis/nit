/// The smallest type that can contain the maximum amount of bits supported for an integer type in this library.
///
/// Currently, the exact limit is 128 bits (as seen in [`MAXIMUM_SUPPORTED_BITS`]).
/// This leaves a potential future expansion to 256 bits, but this isn't a current native integer size, and likely wouldn't be performant, so there isn't anything done for bases above 128.
///
/// # See Also
/// - [`MAXIMUM_SUPPORTED_BITS`]
/// - [`FitsMaximumBitsAsType`]
pub type FitsMaximumBits = u8;
/// The maximum amount of bits supported for an integer type in this library.
/// This also determines the highest base, as anything further would require an equal or more amount of bits to represent.
///
/// # See Also
/// - [`FitsMaximumBits`]
/// - [`FitsMaximumBitsAsType`]
pub const MAXIMUM_SUPPORTED_BITS: FitsMaximumBits = 128;
/// The native integer type utilizing the maximum amount of bits supported by this library.
///
/// # See Also
/// - [`FitsMaximumBits`]
/// - [`MAXIMUM_SUPPORTED_BITS`]
pub type FitsMaximumBitsAsType = u128;

/// The maximum supported base for a number.
pub type BaseMaximum = FitsMaximumBits;

