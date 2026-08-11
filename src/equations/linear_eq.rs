use crate::utils::element_seperator::{ELEMENT_TYPES, Element, MappedEquation, map_elements};

pub fn calculate_linear_eq(eq: &str) {
      let map = map_elements(eq);
    
      if !map.is_equation {
         panic!("Input is not a valid equation");
      }
    let variable = extract_variable_coefficient(eq);
    println!("Linear variable: {}", variable);
    let mut  constant_side:Vec<Element> = Vec::new();
    let mut variable_side:Vec<Element> = Vec::new();
    
    let left_variables_extract = map.left_side.iter()
        .filter(|s| s.element_type == ELEMENT_TYPES[0]);
    let right_variables_extract = map.right_side.as_ref().unwrap_or(&vec![]).iter()
        .filter(|s| s.element_type == ELEMENT_TYPES[0]);
      
     

       


}


fn extract_variable_coefficient(eq: &str) -> String {
    let mut variable = String::new();
    let map: MappedEquation = map_elements(eq);

    let variables = map.left_side.iter()
        .chain(map.right_side.as_deref().unwrap_or(&[]).iter())
        .filter(|s| s.element_type == ELEMENT_TYPES[0]);

    for var in variables {
        for va in var.value.chars() {
            if va.is_alphabetic() {
                if variable.is_empty() {
                    variable = va.to_string();
                } else if variable != va.to_string() {
                    panic!("Multiple variables found: {} and {}", variable, va);
                }
            }
        }
    }

    println!("Extracted variable: {}", variable);
    variable
}
