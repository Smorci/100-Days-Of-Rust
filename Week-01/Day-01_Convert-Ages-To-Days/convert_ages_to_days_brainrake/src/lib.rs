//! Solution to Week-01/Day-01.
//!
//! Convert years to days.

use num::BigUint;

/// Number of days in a year.
pub const YEAR_TO_DAYS: u32 = 365;

/// Returns the number days in a given number of years. A year has [YEAR_TO_DAYS] days. Leap years are not supported.
///
/// # Argument
///
/// * `years` - the number of years to convert to days
///
/// # Example
///
/// ```
/// # use num::BigUint;
/// # use convert_ages_to_days_brainrake::years_to_days;
/// let days = years_to_days(BigUint::from(1_u32));
/// assert_eq!(days, BigUint::from(365_u32));
/// ```
pub fn years_to_days(years: BigUint) -> BigUint {
    years * YEAR_TO_DAYS
}


#[cfg(test)]
mod tests {
    use crate::years_to_days;
    use num::BigUint;
    use std::str::FromStr;

    #[test]
    fn test_0() {
        let actual = years_to_days(BigUint::from(0_u32));
        let expected = BigUint::from(0_u32);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let actual = years_to_days(BigUint::from(1_u32));
        let expected = BigUint::from(365_u32);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_42() {
        let actual = years_to_days(BigUint::from(42_u32));
        let expected = BigUint::from(15330_u32);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_alot() {
        let actual = years_to_days(BigUint::from_str("1000000000000000000000000000000").unwrap());
        let expected = BigUint::from_str("365000000000000000000000000000000").unwrap();
        assert_eq!(actual, expected);
    }
}
