pub const ELEMENT_TYPES: [&str; 7] = [
    "Variable",       
    "Constant",       
    "Exponent",       
    "Bracket",        
    "Power",          
    "Fraction",       
    "Multiplication", 
];

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
    let clean = &parse_equation(equation.trim());
    
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
            left_side: identify_elements(clean),
            right_side: None,
            is_equation: false,
        }
    }
}

pub fn identify_elements(equation: &str) -> Vec<Element> {
    let mut elements: Vec<Element> = Vec::new();

    for token in equation.split_whitespace() {
        if token.is_empty() {
            continue;
        }

       
        if token.contains('*') {
            elements.push(Element {
                value: token.to_string(),
                element_type: ELEMENT_TYPES[6].to_string(), // Multiplication
            });
            continue;
        }

      
        if token == "(" || token == ")" || token == "[" || token == "]" {
            elements.push(Element {
                value: token.to_string(),
                element_type: ELEMENT_TYPES[3].to_string(), // Bracket
            });
            continue;
        }

       
        let body = token.trim_start_matches(|c| c == '+' || c == '-');
        if body.starts_with('(') && body.ends_with(')') && body.len() > 1 {
            elements.push(Element {
                value: token.to_string(),
                element_type: ELEMENT_TYPES[3].to_string(), // Bracket
            });
            continue;
        }

  
        if token.contains('/') {
            elements.push(Element {
                value: token.to_string(),
                element_type: ELEMENT_TYPES[5].to_string(), // Fraction
            });
            continue;
        }

    
        if token.contains("##") {
            elements.push(Element {
                value: token.to_string(),
                element_type: ELEMENT_TYPES[2].to_string(), // Exponent
            });
            continue;
        }

        if token.contains('^') {
            elements.push(Element {
                value: token.to_string(),
                element_type: ELEMENT_TYPES[4].to_string(), // Power
            });
            continue;
        }

        let num_body = token.trim_start_matches(|c| c == '+' || c == '-');
        if !num_body.is_empty() && num_body.chars().all(|c| c.is_ascii_digit() || c == '.') {
            elements.push(Element {
                value: token.to_string(),
                element_type: ELEMENT_TYPES[1].to_string(), // Constant
            });
            continue;
        }

        
        elements.push(Element {
            value: token.to_string(),
            element_type: ELEMENT_TYPES[0].to_string(), // Variable
        });
    }

    elements
}


pub fn parse_equation(data:&str)->String{
    let line = data.replace(" ", "");
    let new_line = line.replace("+", " +").replace("-", " -");
   
    if line.starts_with("-") || line.starts_with("+"){
        return new_line.to_string()[1..].to_string()
    }

    new_line.to_string()
}