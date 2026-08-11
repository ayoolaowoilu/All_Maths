pub mod utils;
pub mod equations;

#[cfg(test)]
mod tests {
    use crate::equations::bodmas::calculate_bodmas;
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

    #[test]
    fn test_bodmas_simple() {
        assert_eq!(calculate_bodmas("-9 +89"), 80.0);
    }

    #[test]
    fn test_bodmas_multiplication() {
        assert_eq!(calculate_bodmas("+2*8"), 16.0);
    }

    #[test]
    fn test_bodmas_fraction() {
        assert!((calculate_bodmas("-7/9") - (-0.777777)).abs() < 0.0001);
    }

    #[test]
    fn test_bodmas_brackets() {
        assert_eq!(calculate_bodmas("+(4-8)"), -4.0);
    }

    #[test]
    fn test_bodmas_full() {
       
        let result = calculate_bodmas("-9 +2*8 -7/9 +89");
        assert!((result - 95.2222).abs() < 0.001);
    }

    #[test]
    fn test_bodmas_nested_bracket() {
        assert_eq!(calculate_bodmas("2 +(3-1)"), 4.0);
    }

    #[test]
    fn test_bodmas_bracket_mult() {
        assert_eq!(calculate_bodmas("(2+3)*4"), 20.0);
    }

    #[test]
    #[should_panic(expected = "expects an expression")]
    fn test_reject_equation() {
        calculate_bodmas("2*x +3 =10");
    }
}