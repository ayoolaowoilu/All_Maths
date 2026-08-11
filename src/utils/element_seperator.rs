pub const OPERATORS: [&str; 6] = ["+", "-", "*", "/", "=", "^"];
pub const ELEMENT_TYPES: [&str; 8] = ["Variable", "Constant", "Operator", "Exponent", "Bracket" , "Power" , "Fraction" , "Multiplication"];

pub struct Element {
    pub value: String,
    pub element_type: String,
}

pub struct MappedEquation {
    pub left_side: Vec<Element>,
    pub right_side: Option<Vec<Element>>,
    pub is_equation: bool,
}

pub fn map_elements(equation: &str) -> MappedEquation {
    let clean = equation;
    
    if clean.starts_with('*') || clean.starts_with('/') {
        panic!("Equation cannot start with * or /");
    }

    let is_equation = clean.contains('=');
    
    if is_equation {
        let parts: Vec<&str> = clean.split('=').collect();
        if parts.len() != 2 {
            panic!("Invalid equation: multiple = signs");
        }
        MappedEquation {
            left_side: identify_elements(parts[0]),
            right_side: Some(identify_elements(parts[1])),
            is_equation: true,
        }
    } else {
        MappedEquation {
            left_side: identify_elements(&clean),
            right_side: None,
            is_equation: false,
        }
    }
}


pub fn identify_elements(equation:&str) -> Vec<Element> {
     
     let clean = equation;
     let mut elements:Vec<Element> = Vec::new();
     let data:Vec<&str> = clean.split_whitespace().collect();
     
     for el in data {

        if el.starts_with("(") && el.ends_with(")") {
             elements.push(Element { value: el.to_string(), element_type: ELEMENT_TYPES[4].to_string() });
        }
        
         if el.contains("*") {
            elements.push(Element { value: el.to_string(), element_type: ELEMENT_TYPES[7].to_string() });
         }else if el.contains("/") {
             elements.push(Element { value: el.to_string(), element_type: ELEMENT_TYPES[6].to_string() });
         }else if el.contains("##") {
             elements.push(Element { value: el.to_string(), element_type: ELEMENT_TYPES[3].to_string() });
         }else {
            elements.push(Element { value: el.to_string(), element_type: ELEMENT_TYPES[1].to_string() });
         }
     }

     elements
  


}