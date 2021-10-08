use enumerare::DefaultEnum;

#[test]
fn test_explicit_default() {
	#[derive(DefaultEnum, Debug, PartialEq)]
	enum Test {
		Variant,
		#[default]
		VariantTwo,
	}

	assert_eq!(Test::VariantTwo, Test::default());
	assert_ne!(Test::Variant, Test::default());
}

#[test]
fn test_implied_first_default() {
	#[derive(DefaultEnum, Debug, PartialEq)]
	enum Test {
		Variant,
		VariantTwo,
	}

	assert_eq!(Test::Variant, Test::default());
	assert_ne!(Test::VariantTwo, Test::default());
}
