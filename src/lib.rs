mod auxilliary;

mod fractions;
pub use crate::fractions::Fraction;

#[macro_use]
mod macros;

#[cfg(test)]
mod tests {
    use crate::fractions::Fraction;

    #[test]
    fn fraction_reduces_correctly() {
        let f = frac!(18, 512);

        let expected_equivalent = frac!(9, 256);
        assert_eq!(expected_equivalent, f);
    }

    #[test]
    #[should_panic]
    fn fraction_should_panic_with_zero_denominator() {
        frac!(1, 0);
    }

    #[test]
    fn fraction_can_be_obtained_as_tuple() {
        let f = frac!(7, 23);

        let expected_tuple = (7, 23);
        assert_eq!(expected_tuple, f.get_as_tuple());
    }

    #[test]
    fn fraction_can_be_checked_for_properness() {
        let improper = frac!(10, 9);
        let proper = frac!(3, 4);

        assert!(!improper.is_proper());
        assert!(proper.is_proper());
    }

    #[test]
    fn fraction_is_reversed_correctly() {
        let f = frac!(3, 5);

        let expected_result = frac!(5, 3);
        assert_eq!(expected_result, f.reverse());

        let f = frac!(-3, 5);

        let expected_result = frac!(-5, 3);
        assert_eq!(expected_result, f.reverse());
    }

    #[test]
    #[should_panic]
    fn fraction_should_panic_when_zero_reversed() {
        frac!(0).reverse();
    }

    #[test]
    fn fraction_is_displayed_correctly() {
        let f = frac!(3, 13);

        let expected_result = "3/13";
        assert_eq!(expected_result, format!("{}", f));
    }

    #[test]
    fn sign_is_transfered_to_numerator() {
        let f = frac!(1, -5);

        let expected_equivalent = frac!(-1, 5);
        assert_eq!(expected_equivalent, f);
    }

    #[test]
    fn fractions_are_added_correctly() {
        let f = frac!(1, 14);
        let g = frac!(3, 35);

        let expected_result = frac!(11, 70);
        assert_eq!(expected_result, f + g);
    }

    #[test]
    fn fractions_are_subtracted_correctly() {
        let f = frac!(1, 7);
        let g = frac!(3, 35);

        let expected_result = frac!(2, 35);
        assert_eq!(expected_result, f - g);
    }

    #[test]
    fn fractions_are_negated_correctly() {
        let f = frac!(5, 19);

        let expected_result = frac!(-5, 19);
        assert_eq!(expected_result, -f);
    }

    #[test]
    fn fractions_are_multiplied_correctly() {
        let f = frac!(3, 13);
        let g = frac!(4, 11);

        let expected_result = frac!(12, 143);
        assert_eq!(expected_result, f * g);
    }

    #[test]
    fn fractions_are_divided_correctly() {
        let f = frac!(1, 19);
        let g = frac!(5, 8);

        let expected_result = frac!(8, 95);
        assert_eq!(expected_result, f / g);
    }

    #[allow(unused_must_use)]
    #[test]
    #[should_panic]
    fn fraction_should_panic_when_divided_by_zero() {
        let f = frac!(3, 10);
        let g = frac!(0);

        f / g;
    }
}
