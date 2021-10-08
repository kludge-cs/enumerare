#![allow(dead_code)]

use enumerare::{DefaultEnum, SizedEnum};

#[test]
fn test_empty_enum() {
	#[derive(SizedEnum, Debug)]
	enum Test {}

	assert_eq!(Test::VARIANTS, 0usize);
}

#[test]
fn test_enum_sizes() {
	#[derive(SizedEnum, Debug)]
	enum Test {
		Variant,
	}
	#[derive(SizedEnum, Debug)]
	enum TestTwo {
		Variant,
		VariantTwo,
	}

	assert_eq!(Test::VARIANTS, 1usize);
	assert_eq!(TestTwo::VARIANTS, 2usize);
}

#[test]
fn test_enum_with_attrs() {
	#[derive(DefaultEnum, SizedEnum, Debug)]
	enum Test {
		Variant,
		#[default]
		VariantTwo,
	}

	assert_eq!(Test::VARIANTS, 2usize);
}
