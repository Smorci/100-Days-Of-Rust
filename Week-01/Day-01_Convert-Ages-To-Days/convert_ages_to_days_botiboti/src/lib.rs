use num_bigint::BigUint;

pub const DAYS_IN_YEAR: u64 = 365;

// turn age in years to days
pub fn calc_days(age: &BigUint) -> BigUint {
    age * DAYS_IN_YEAR
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_days_for_earth() {
        assert_eq!(
            calc_days(&BigUint::from(4500000000_u64)),
            BigUint::from(1642500000000_u64)
        );
    }

    #[test]
    fn calc_zero() {
        assert_eq!(calc_days(&BigUint::from(0_u32)), BigUint::from(0_u32));
    }

    #[test]
    fn calc_one() {
        assert_eq!(calc_days(&BigUint::from(1_u32)), BigUint::from(365_u32));
    }

    #[test]
    fn calc_hundred() {
        assert_eq!(calc_days(&BigUint::from(100_u32)), BigUint::from(36500_u32));
    }
}
