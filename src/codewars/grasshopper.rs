// This function should test if the factor is a factor of base.

// Return true if it is a factor or false if it is not.

// About factors
// Factors are numbers you can multiply together to get another number.

// 2 and 3 are factors of 6 because: 2 * 3 = 6

// You can find a factor by dividing numbers. If the remainder is 0 then the number is a factor.
// You can use the mod operator (%) in most languages to check for a remainder
// For example 2 is not a factor of 7 because: 7 % 2 = 1

// Note: base is a non-negative number, factor is a positive number.

pub fn check_for_factor(base: i32, factor: i32) -> bool {
    if base % factor == 0 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(check_for_factor(10, 2), true);
        assert_eq!(check_for_factor(63, 7), true);
        assert_eq!(check_for_factor(2450, 5), true);
        assert_eq!(check_for_factor(24612, 3), true);
        assert_eq!(check_for_factor(9, 2), false);
        assert_eq!(check_for_factor(653, 7), false);
        assert_eq!(check_for_factor(2453, 5), false);
        assert_eq!(check_for_factor(24617, 3), false);
    }
}
