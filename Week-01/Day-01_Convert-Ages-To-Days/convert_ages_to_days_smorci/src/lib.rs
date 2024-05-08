//! Convert years to days.
//!
//! Provides a way to return the number of days in a year
use num_bigint::BigUint;

/// Number of days in a year
pub const NUMBER_OF_DAYS_IN_A_YEAR: u32 = 365;

/// Takes a year as an argument and returns the equivalent in days
/// ```
/// use num_bigint::BigUint;
/// use std::str::FromStr;
/// use convert_ages_to_days_smorci::convert_years_to_days;
/// assert_eq!(
///     convert_years_to_days(&BigUint::from(10_u32)),
///     BigUint::from_str("3650").unwrap()
/// );
/// assert_eq!(
///     convert_years_to_days(&BigUint::from(1_u32)),
///     BigUint::from_str("365").unwrap()
/// );
/// assert_eq!(
///     convert_years_to_days(&BigUint::from(40_u32)),
///     BigUint::from_str("14600").unwrap()
/// );
///
/// ```
pub fn convert_years_to_days(years: &BigUint) -> BigUint {
    NUMBER_OF_DAYS_IN_A_YEAR * years
}

// Unit tests
#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;

    // Tests a number bigger than u64::MAX
    #[test]
    fn test_big_integer() {
        let test_number = BigUint::from_str("20000000000000000000").unwrap();
        assert_eq!(
            convert_years_to_days(&test_number),
            BigUint::from_str("7300000000000000000000").unwrap()
        );
    }

}
