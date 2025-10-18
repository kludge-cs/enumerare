#![deny(
	missing_docs,
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	clippy::missing_safety_doc
)]
#![allow(clippy::tabs_in_doc_comments)]

//! A collection of utilities for working with enums.

use core::{
	fmt,
	fmt::{Display, Formatter},
};

#[cfg(feature = "enumerare-macros")]
#[doc(hidden)]
pub use enumerare_macros::*;

/// A derivable trait for cyclic linear populated enums.
///
/// A cyclic enum is one whose variants can be iterated through bidirectionally
/// and wraps around at each boundary.
///
/// # Derivable
///
/// It is preferable to `derive` this trait instead of implementing it yourself.
/// You can use this trait with `#[derive]` for any enum so long as it has more
/// than one variant (i.e. its bit representation is greater than zero), none of
/// its variants have alternate discriminants (i.e. it starts at 0 and each
/// variant increments in value by 1) and all of its variants are unit variants.
///
/// When it is derived, you get a free implementation of [`SizedEnum`] and
/// [`TryFrom<usize>`][core::convert::TryFrom].
pub trait Cycle: SizedEnum {
	/// Returns a result containing the variant at the specified index, or a
	/// [`CycleError::OutOfBounds`] if the index was out of bounds. This method
	/// is intended for custom error handling - generally, if you can ensure a
	/// correct index, it is easier to use [`Cycle::cycle_to`].
	///
	/// # Errors
	///
	/// This errors if the specified index was out of bounds for the enum. See
	/// [`CycleError::OutOfBounds`] for more information.
	///
	/// # Examples
	///
	/// ```
	/// # use enumerare::{Cycle, CycleError};
	/// #[derive(Cycle, Debug, PartialEq)]
	/// enum Kind {
	/// 		A,
	/// 		B,
	/// 		C,
	/// 	}
	/// 	assert_eq!(Kind::A.try_cycle_to(0)?, Kind::A);
	/// 	assert_eq!(Kind::A.try_cycle_to(1)?, Kind::B);
	/// 	assert_eq!(Kind::A.try_cycle_to(2)?, Kind::C);
	/// 	assert_eq!(Kind::A.try_cycle_to(3), Err(CycleError::OutOfBounds));
	/// 	# Ok::<(), CycleError>(())
	/// ```
	/// No matter what variant this is called on, it behaves the same.
	/// ```
	/// # use enumerare::{Cycle, CycleError};
	/// # #[derive(Cycle, Debug, PartialEq)]
	/// # enum Kind { A, B, C, }
	/// 	assert_eq!(Kind::B.try_cycle_to(0)?, Kind::A);
	/// 	assert_eq!(Kind::B.try_cycle_to(1)?, Kind::B);
	/// 	assert_eq!(Kind::B.try_cycle_to(2)?, Kind::C);
	/// 	assert_eq!(Kind::B.try_cycle_to(3), Err(CycleError::OutOfBounds));
	/// 	# Ok::<(), CycleError>(())
	/// ```
	///
	/// # Roadmap
	///
	/// In future, this will be a static (`self`less) method of another trait.
	fn try_cycle_to(self, idx: usize) -> Result<Self, CycleError>;
	/// Returns the variant at the specified index, or panics if the index was
	/// out of bounds. This is semantically equivalent to unwrapping the result
	/// of [`Cycle::try_cycle_to`].
	///
	/// # Panics
	///
	/// This panics if the specified index was out of bounds for the enum. See
	/// [`CycleError::OutOfBounds`] for more information.
	///
	/// ```should_panic
	/// # use enumerare::Cycle;
	/// #[derive(Cycle, Debug, PartialEq)]
	/// enum Kind {
	/// 		A,
	/// 		B,
	/// 		C,
	/// 	}
	/// 	let x = Kind::A;
	/// 	// kind has 3 variants so the highest index is 2 so this will panic
	/// 	x.cycle_to(5);
	/// ```
	///
	/// # Examples
	///
	/// ```
	/// # use enumerare::Cycle;
	/// #[derive(Cycle, Debug, PartialEq)]
	/// enum Kind {
	/// 		A,
	/// 		B,
	/// 		C,
	/// 	}
	/// 	assert_eq!(Kind::A.cycle_to(0), Kind::A);
	/// 	assert_eq!(Kind::A.cycle_to(1), Kind::B);
	/// 	assert_eq!(Kind::A.cycle_to(2), Kind::C);
	/// 	```
	/// 	No matter what variant this is called on, it behaves the same.
	/// 	```
	/// # use enumerare::Cycle;
	/// # #[derive(Cycle, Debug, PartialEq)]
	/// # enum Kind { A, B, C, }
	/// 	assert_eq!(Kind::B.cycle_to(0), Kind::A);
	/// 	assert_eq!(Kind::B.cycle_to(1), Kind::B);
	/// 	assert_eq!(Kind::B.cycle_to(2), Kind::C);
	/// 	```
	///
	/// # Roadmap
	///
	/// In future, this will be a static (`self`less) method of another trait.
	fn cycle_to(self, idx: usize) -> Self;
	/// Returns the variant `step` variants ahead if `step` is positive, or
	/// `step` variants behind if `step` is negative, wrapping at the first and
	/// last variant.
	fn cycle_by(self, step: isize) -> Self;
	/// Returns the next variant. See [`Cycle::cycle_by`].
	///
	/// Note that this wraps at the last variant.
	///
	/// # Examples
	///
	///	```
	/// # use enumerare::Cycle;
	/// #[derive(Cycle, Debug, PartialEq)]
	/// enum Kind {
	///		A,
	///		B,
	///		C,
	///	}
	///	assert_eq!(Kind::A.next(), Kind::B);
	///	assert_eq!(Kind::B.next(), Kind::C);
	///	assert_eq!(Kind::C.next(), Kind::A);
	///	```
	fn next(self) -> Self;
	/// Returns the previous variant. See [`Cycle::cycle_by`].
	///
	/// Note that this wraps at the first variant.
	///
	/// # Examples
	///
	///	```
	/// # use enumerare::Cycle;
	/// #[derive(Cycle, Debug, PartialEq)]
	/// enum Kind {
	///		A,
	///		B,
	///		C,
	///	}
	///	assert_eq!(Kind::C.prev(), Kind::B);
	///	assert_eq!(Kind::B.prev(), Kind::A);
	///	assert_eq!(Kind::A.prev(), Kind::C);
	///	```
	fn prev(self) -> Self;
}

/// A list specifying categories of errors produced by [`Cycle::try_cycle_to`].
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CycleError {
	/// The provided index was out of bounds, i.e. outside of `0..n` where `n`
	/// is [`SizedEnum::VARIANTS`].
	OutOfBounds,
}

impl Display for CycleError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "enum failed to cycle")
	}
}

unsafe impl Send for CycleError {}
unsafe impl Sync for CycleError {}

/// A derivable marker trait for populated enums that have default variants.
///
/// # Derivable
///
/// This can be used with `#[derive]` in one of two ways - with the `#[default]`
/// attribute for an explicit default, or alone for an implicit default. Under
/// the hood, this generates a free [`Default`] implementation for you. This
/// trait can be used with `#[derive]` for any enum whose default variant is
/// a unit variant.
///
/// It is a model solution for [RFC 3017 derive_enum_default][rfc-3017].
///
/// [rfc-3017]: https://rust-lang.github.io/rfcs/3107-derive-enum-default.html#derivedefault
///
/// ## Explicit defaults
///
/// Explicit defaults use the `#[default]` attribute on variants to denote the
/// default enum variant for the generated [`Default`] implementation.
///
/// ```
/// # use enumerare::DefaultEnum;
/// #[derive(DefaultEnum)]
/// enum Kind {
/// 		A,
/// 		#[default] B,
/// 		C,
/// }
/// ```
///
/// This would generate the following [`Default`] implementation for you:
///
/// ```
/// # enum Kind { B }
/// impl Default for Kind {
/// 		fn default() -> Kind { Kind::B }
/// 	}
/// 	```
///
/// 	## Implicit defaults
///
/// You can `derive` [`DefaultEnum`] without using the `#[default]` attribute to
/// select the first variant as an assumed default.
/// ```
/// # use enumerare::DefaultEnum;
/// #[derive(DefaultEnum)]
/// enum Kind {
///		A,
///		B,
///		C,
/// }
/// ```
/// 
/// This would generate the following [`Default`] implementation for you:
/// ```
/// # enum Kind { A }
/// impl Default for Kind {
///		fn default() -> Kind { Kind::A }
/// }
/// ```
pub trait DefaultEnum: Default {}

/// A derivable trait for enums with public variant quantities, useful to
/// determine their bit size or ensure valid indexes.
///
/// # Derivable
///
/// You can use this trait with `#[derive]` for any enum, even empty ones.
///
/// # Examples
///
/// ```
/// # use enumerare::SizedEnum;
/// #[derive(SizedEnum)]
/// enum Kind {
/// 		A,
/// 		B,
/// 		C,
/// 	}
///
/// 	assert_eq!(Kind::VARIANTS, 3usize);
/// ```
///
/// ```
/// # use enumerare::SizedEnum;
/// #[derive(SizedEnum)]
/// enum Kind {}
///
/// 	assert_eq!(Kind::VARIANTS, 0usize);
/// ```
pub trait SizedEnum: Sized {
	/// The number of variants the enum has.
	const VARIANTS: usize;
}
