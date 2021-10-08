use core::ops::Not;

use proc_macro2::{Ident as Ident2, Span, TokenStream as TokenStream2};
use quote::format_ident;
use syn::{Data, DataEnum, Error, Fields, Result, Variant};

macro_rules! gen_err {
	($name:ident, $err:literal) => {
		pub fn $name() -> Error {
			Error::new(Span::call_site(), $err)
		}
	};
}

gen_err!(non_enum_err, "This macro only supports enums.");
gen_err!(unpopulated_enum_err, "This macro only supports populated enums.");
gen_err!(
	non_linear_unpopulated_enum_err,
	"This macro only supports linear populated enums."
);
gen_err!(non_linear_enum_err, "This macro only supports linear enums.");
gen_err!(non_variant_err, "This macro could not find any valid variants.");
gen_err!(non_unit_variant_err, "This macro only supports unit variants.");

pub fn uint_of_size(size: usize) -> Ident2 {
	format_ident!("u{}", size)
}

pub fn primitive_bit_req(n: usize) -> usize {
	(usize::BITS - n.leading_zeros()).next_power_of_two().clamp(8, usize::BITS) as usize

	// //TODO: Cleaner variant for when int_log is stabilized:
	// (n.log2().next_power_of_two() as usize).clamp(8, size_of::<usize>())
}

pub fn to_compile_err(err: Error) -> TokenStream2 {
	err.to_compile_error()
}

pub fn unwrap_enum(data: &Data) -> Result<DataEnum> {
	match data {
		Data::Enum(data_enum) => Ok(data_enum.to_owned()),
		_ => Err(non_enum_err()),
	}
}

pub fn unwrap_populated_enum(data: &Data) -> Result<DataEnum> {
	let data = unwrap_enum(data)?;
	data.variants.len().gt(&0).then(|| data).ok_or_else(unpopulated_enum_err)
}

pub fn unwrap_linear_enum(data: &Data) -> Result<DataEnum> {
	let data = unwrap_enum(data)?;
	data.variants
		.clone()
		.iter()
		.any(|variant| variant.discriminant.is_some())
		.not()
		.then(|| data)
		.ok_or_else(non_linear_enum_err)
}

pub fn unwrap_linear_populated_enum(data: &Data) -> Result<DataEnum> {
	unwrap_populated_enum(data)
		.and_then(|_| unwrap_linear_enum(data))
		.map_err(|_| non_linear_unpopulated_enum_err())
}

pub fn unwrap_variant(variant: Option<&Variant>) -> Result<Variant> {
	variant.map(|x| x.clone()).ok_or_else(non_variant_err)
}

pub fn unwrap_unit_variant(variant: Option<&Variant>) -> Result<Variant> {
	let variant = unwrap_variant(variant)?;
	match variant.fields {
		Fields::Unit => Ok(variant),
		_ => Err(non_unit_variant_err()),
	}
}
