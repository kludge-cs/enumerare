mod helpers;
mod macros;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use helpers::to_compile_err;
use macros::{derive_cycle, derive_default, derive_sized_enum};

#[proc_macro_derive(Cycle)]
pub fn derive_cycle_macro(input: TokenStream) -> TokenStream {
	derive_cycle(&parse_macro_input!(input as DeriveInput))
		.unwrap_or_else(to_compile_err)
		.into()
}

#[proc_macro_derive(DefaultEnum, attributes(default))]
pub fn derive_default_macro(input: TokenStream) -> TokenStream {
	derive_default(&parse_macro_input!(input as DeriveInput))
		.unwrap_or_else(to_compile_err)
		.into()
}

#[proc_macro_derive(SizedEnum)]
pub fn derive_sized_enum_macro(input: TokenStream) -> TokenStream {
	derive_sized_enum(&parse_macro_input!(input as DeriveInput))
		.unwrap_or_else(to_compile_err)
		.into()
}
