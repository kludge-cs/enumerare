use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Result};

use crate::helpers::*;

macro_rules! macro_safe {
	($name:ident, $path:ty) => {
		let $name = quote! { $path };
	};
}

pub(crate) fn derive_cycle(ast: &DeriveInput) -> Result<TokenStream2> {
	let data = unwrap_linear_populated_enum(&ast.data)?;
	let name = &ast.ident;
	let impl_sized = derive_sized_enum(ast)?;
	let n = data.variants.len();
	let nisize = n as isize;
	let size = uint_of_size(primitive_bit_req(n));

	let (impl_gens, ty_gens, where_cl) = ast.generics.split_for_impl();
	macro_safe!(ms_transmute, ::core::mem::transmute);
	macro_safe!(ms_usize, ::core::primitive::usize);
	macro_safe!(ms_isize, ::core::primitive::isize);
	macro_safe!(ms_try_from, ::core::convert::TryFrom);
	macro_safe!(ms_try_into, ::core::convert::TryInto);
	macro_safe!(ms_result, ::core::result::Result);

	Ok(quote! {
		#impl_sized

		impl #impl_gens #ms_try_from<#ms_usize> for #name #ty_gens #where_cl {
			type Error = ::enumerare::CycleError;
			fn try_from(idx: #ms_usize) -> #ms_result<#name, <#name as #ms_try_from<#ms_usize>>::Error> {
				(0..#n)
					.contains(&idx)
					.then(|| unsafe { #ms_transmute(idx as #size) })
					.ok_or_else(|| ::enumerare::CycleError::OutOfBounds)
			}
		}

		impl #impl_gens ::enumerare::Cycle for #name #ty_gens #where_cl {
			fn try_cycle_to(self, idx: #ms_usize) -> #ms_result<#name, ::enumerare::CycleError> {
				use #ms_try_into;
				idx.try_into()
			}

			fn cycle_to(self, idx: #ms_usize) -> #name {
				self.try_cycle_to(idx).unwrap()
			}

			fn cycle_by(self, step: #ms_isize) -> #name {
				use #ms_try_into;
				(self as #ms_isize + step)
					.rem_euclid(#nisize)
					.unsigned_abs()
					.try_into()
					.unwrap()
			}

			fn next(self) -> #name {
				self.cycle_by(1)
			}

			fn prev(self) -> #name {
				self.cycle_by(-1)
			}
		}
	})
}

pub(crate) fn derive_default(ast: &DeriveInput) -> Result<TokenStream2> {
	let data = unwrap_populated_enum(&ast.data)?;
	let name = &ast.ident;
	let variant = unwrap_unit_variant(
		data.variants
			.iter()
			.find(|variant| {
				variant.attrs.iter().any(|attr| attr.path().is_ident("default"))
			})
			.or_else(|| data.variants.first()),
	)?
	.ident;

	let (impl_gens, ty_gens, where_cl) = ast.generics.split_for_impl();
	macro_safe!(ms_default, ::core::default::Default);

	Ok(quote! {
		impl #impl_gens ::enumerare::DefaultEnum for #name #ty_gens #where_cl {}

		impl #impl_gens #ms_default for #name #ty_gens #where_cl {
			fn default() -> #name {
				#name::#variant
			}
		}
	})
}

pub(crate) fn derive_sized_enum(ast: &DeriveInput) -> Result<TokenStream2> {
	let data = unwrap_enum(&ast.data)?;
	let name = &ast.ident;
	let n = data.variants.len();

	let (impl_gens, ty_gens, where_cl) = ast.generics.split_for_impl();

	Ok(quote! {
		impl #impl_gens ::enumerare::SizedEnum for #name #ty_gens #where_cl {
			const VARIANTS: usize = #n;
		}
	})
}
