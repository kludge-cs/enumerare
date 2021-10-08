#![allow(dead_code)]

use enumerare::{Cycle, CycleError};

#[test]
fn test_try_cycle_to() {
	#[derive(Cycle, PartialEq, Debug)]
	enum Test {
		Variant,
		VariantTwo,
	}

	// Test normal try_cycle_to
	assert_eq!(Test::Variant.try_cycle_to(0).unwrap(), Test::Variant);
	assert_eq!(Test::Variant.try_cycle_to(1).unwrap(), Test::VariantTwo);
	assert_eq!(Test::VariantTwo.try_cycle_to(0).unwrap(), Test::Variant);
	assert_eq!(Test::VariantTwo.try_cycle_to(1).unwrap(), Test::VariantTwo);

	// Test error case of try_cycle_to - idx out of enum bounds
	assert_eq!(
		Test::Variant.try_cycle_to(2).unwrap_err(),
		CycleError::OutOfBounds
	);
	assert_eq!(
		Test::Variant.try_cycle_to(99).unwrap_err(),
		CycleError::OutOfBounds
	);
	assert_eq!(
		Test::VariantTwo.try_cycle_to(2).unwrap_err(),
		CycleError::OutOfBounds
	);
	assert_eq!(
		Test::VariantTwo.try_cycle_to(99).unwrap_err(),
		CycleError::OutOfBounds
	);
}

#[test]
fn test_cycle_to() {
	#[derive(Cycle, PartialEq, Debug)]
	enum Test {
		Variant,
		VariantTwo,
	}

	assert_eq!(Test::Variant.cycle_to(0), Test::Variant);
	assert_eq!(Test::Variant.cycle_to(1), Test::VariantTwo);
	assert_eq!(Test::VariantTwo.cycle_to(0), Test::Variant);
	assert_eq!(Test::VariantTwo.cycle_to(1), Test::VariantTwo);
}

#[test]
#[should_panic]
fn test_cycle_to_panic() {
	#[derive(Cycle, PartialEq, Debug)]
	enum Test {
		Variant,
		VariantTwo,
	}

	// Test panicking on idx out of enum bounds for cycle_to
	Test::Variant.cycle_to(2);
	Test::VariantTwo.cycle_to(2);
}

#[test]
fn test_cycle_by() {
	#[derive(Cycle, PartialEq, Debug)]
	enum Test {
		Variant,
		VariantTwo,
		VariantThree,
	}

	// Test ability to cycle forwards
	assert_eq!(Test::Variant.cycle_by(1), Test::VariantTwo);
	assert_eq!(Test::VariantThree.cycle_by(1), Test::Variant);

	// Test ability to cycle backwards
	assert_eq!(Test::Variant.cycle_by(-1), Test::VariantThree);
	assert_eq!(Test::VariantThree.cycle_by(-1), Test::VariantTwo);

	// Test cycle wrapping
	assert_eq!(Test::Variant.cycle_by(3), Test::Variant);
	assert_eq!(Test::VariantTwo.cycle_by(5), Test::Variant);
	assert_eq!(Test::VariantThree.cycle_by(20), Test::VariantTwo);
}

#[test]
fn test_next_prev_congruence() {
	#[derive(Cycle, PartialEq, Debug)]
	enum Test {
		Variant,
		VariantTwo,
		VariantThree,
	}

	assert_eq!(Test::Variant.next(), Test::Variant.cycle_by(1));
	assert_eq!(Test::Variant.prev(), Test::Variant.cycle_by(-1));
}
