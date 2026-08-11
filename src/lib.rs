pub mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::element_seperator::{identify_elements, map_elements};

    #[test]
    fn test_basic_equation_mapping() {
        // Match YOUR format: no standalone -, signs attached to terms
        let equation = "3 +4*2 /(1-5) ^2 ^3";
        let mapped = map_elements(equation);
        
        // map_elements returns MappedEquation with spaced terms
        assert_eq!(mapped.left_side.len(), 3);
        assert!(!mapped.is_equation);
        assert!(mapped.right_side.is_none());
    }

    #[test]
    fn test_equation_with_equals() {
        let mapped = map_elements("2*x +3 =10");
        assert!(mapped.is_equation);
        assert!(mapped.right_side.is_some());
        assert_eq!(mapped.left_side.len(), 2);   // 2*x, +3
        assert_eq!(mapped.right_side.unwrap().len(), 1); // 10
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
}