#![allow(unused_macros, unused_imports)]

/// Defers the parsing of a token temporarily.
/// This is useful for the usage of conditional only nightly-only syntax, which will still prevent compilation on stable even if behind a `#[cfg(...)]` attribute.
macro_rules! defer {
	($($token: tt)*) => {
		$($token)*
	}
}

/// Defines a trait, doing so as `const` if the nightly feature is enabled.
macro_rules! define_const_trait {
	($(#[$meta: meta])* $vis: vis $ident: ident $($details: tt)*) => {
		#[cfg(feature = "nightly")]
		crate::internal_macros::defer!{ $(#[$meta])* #[const_trait] $vis trait $ident $($details)* }
		#[cfg(not(feature = "nightly"))]
		$(#[$meta])* $vis trait $ident $($details)*
	};
}

/// Defines a function, doing so as `const` if the nightly feature is enabled.
macro_rules! define_const_func {
	($(#[$meta: meta])* $vis: vis $ident: ident $($details: tt)*) => {
		#[cfg(feature = "nightly")]
		$(#[$meta])* $vis const fn $ident $($details)*
		#[cfg(not(feature = "nightly"))]
		$(#[$meta])* $vis fn $ident $($details)*
	};
}

/// Implements a trait for a struct, doing so as `const` if the nightly feature is enabled.
macro_rules! const_impl {
	($trait: ty | $struct: ty { $($impl: tt)* }) => {
		#[cfg(feature = "nightly")]
		crate::internal_macros::defer!{ impl const $trait for $struct { $($impl)* } }
		#[cfg(not(feature = "nightly"))]
		impl $trait for $struct { $($impl)* }
	};
	($trait: ty | $struct: ty) => {
		const_impl_base!($trait | $struct {});
	};
}

/// Implements a trait for a struct, doing so as `const` if the nightly feature is enabled.
/// This variant also creates a constant generic parameter `BASE` for the base of the number.
macro_rules! const_impl_base {
	($trait: ty | $struct: ty { $($impl: tt)* }) => {
		#[cfg(feature = "nightly")]
		defer!{ impl<const BASE: BaseMaximum> const $trait for $struct { $($impl)* } }
		#[cfg(not(feature = "nightly"))]
		impl<const BASE: BaseMaximum> $trait for $struct { $($impl)* }
	};
	($trait: ty | $struct: ty) => {
		const_impl_base!($trait | $struct {});
	};
}

/// Derives a set of traits for a struct, doing so as `const` if the nightly feature is enabled.
macro_rules! deriving_const {
	(($($traits: ident),*) for { $($struct: tt)* }) => {
		#[cfg_attr(feature = "nightly", derive_const($($traits),*))]
		#[cfg_attr(not(feature = "nightly"), derive($($traits),*))]
		$($struct)*
	}
}

/// Attempts to implement the `Error` trait for the given struct.
///
/// - If a nightly compiler is used and [`error-in-core`] is enabled, it will use [`core::error::Error`].
/// - Otherwise, if `std` is enabled, it will use [`std::error::Error`].
///
/// [`error-in-core`]: https://github.com/rust-lang/rust/issues/103765
macro_rules! impl_error {
	($ident: ident) => {
		#[cfg(all(feature = "nightly", feature = "error-in-core"))]
		impl core::error::Error for $ident {}
		#[cfg(all(feature = "std", not(feature = "error-in-core")))]
		impl std::error::Error for $ident {}
	};
}

/// Define an empty error type with the given name and string, which will be used as the error message.
/// Additional attributes (and documentation) can be added to the struct after the string.
macro_rules! define_empty_error {
	($name: ident, $string: literal $(, $(#[$attr: meta])*)?) => {
		$($(#[$attr])*)?
		#[derive(Debug)]
		pub struct $name;
		impl core::fmt::Display for $name {
			#[cfg(not(tarpaulin_include))]
			fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
				write!(f, $string)
			}
		}
		crate::internal_macros::impl_error!($name);
	}
}

pub(crate) use defer;
pub(crate) use define_const_trait;
pub(crate) use define_const_func;
pub(crate) use const_impl;
pub(crate) use const_impl_base;
pub(crate) use deriving_const;
pub(crate) use impl_error;
pub(crate) use define_empty_error;
