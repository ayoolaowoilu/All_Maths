pub mod utils;
pub mod equations;

#[cfg(test)]
mod tests {
    use crate::equations::pemdas::calculate_pemdas;
    use crate::utils::element_seperator::{identify_elements, map_elements};

    #[test]
    fn test_basic_equation_mapping() {
        // Match YOUR format: no standalone -, signs attached to terms
        let equation = "3 +4*2 /(1-5) ^2 ^3";
        let mapped = map_elements(equation);
        
        // map_elements returns MappedEquation with spaced terms
        assert_eq!(mapped.left_side.len(), 2);
        assert!(!mapped.is_equation);
        assert!(mapped.right_side.is_none());
    }

    #[test]
    fn test_equation_with_equals() {
        let mapped = map_elements("2*x +3 =10");
        assert!(mapped.is_equation);
        assert!(mapped.right_side.is_some());
        assert_eq!(mapped.left_side.len(), 2);   // 2*x, +3
        assert_eq!(mapped.right_side.as_ref().unwrap().len(), 1); // 10
    }

    #[test]
    fn test_pemdas_result_fields_are_accessible() {
        let result = calculate_pemdas("1+2 = 3");
        assert_eq!(result.left, 3.0);
        assert_eq!(result.right, Some(3.0));
        assert!(result.is_equation);
    }

    #[test]
    #[should_panic(expected = "cannot start with")]
    fn test_panic_on_bad_start() {
        map_elements("*2 +3");
    }

    #[test]
    fn test_multiplication_priority() {
        let elements = identify_elements("+2*8 -7/9 +89");
        assert_eq!(elements[0].element_type, "Multiplication"); // +2*8
        assert_eq!(elements[1].element_type, "Fraction");        // -7/9
        assert_eq!(elements[2].element_type, "Constant");          // +89
    }

    #[test]
    fn test_bracket_with_sign() {
        let elements = identify_elements("+(4-8)");
        assert_eq!(elements[0].element_type, "Bracket");
        assert_eq!(elements[0].value, "+(4-8)");
    }

    #[test]
    fn test_pemdas_md_before_as() {
        // 2 + 12 = 14, not (2+3)*4 = 20
        let r = calculate_pemdas("2 +3*4");
        assert_eq!(r.left, 14.0);
    }

    #[test]
    fn test_pemdas_parentheses_first() {
        // (2+3) = 5, *4 = 20
        let r = calculate_pemdas("(2+3)*4");
        assert_eq!(r.left, 20.0);
    }

    #[test]
    fn test_pemdas_exponents() {
        // 2^(3^2) = 2^9 = 512 (right-associative)
        let r = calculate_pemdas("2^3^2");
        assert_eq!(r.left, 512.0);
    }

    #[test]
    fn test_equation_both_sides() {
        let r = calculate_pemdas("2*5 +3 = 13");
        assert!(r.is_equation);
        assert_eq!(r.left, 13.0);
        assert_eq!(r.right, Some(13.0));
    }

    #[test]
    fn test_fraction_and_mult() {
        // 16 - 0.777... + 89
        let r = calculate_pemdas("+2*8 -7/9 +89");
        assert!((r.left - 104.222).abs() < 0.01);
    }
}