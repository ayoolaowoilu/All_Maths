use crate::utils::element_seperator::{map_elements, Element};

pub struct PemdasResult {
    pub left: f64,
    pub right: Option<f64>,
    pub is_equation: bool,
}

pub fn calculate_pemdas(equation: &str) -> PemdasResult {
    let mapped = map_elements(equation);
    
    PemdasResult {
        left: evaluate_side(&mapped.left_side),
        right: mapped.right_side.as_ref().map(|side| evaluate_side(side)),
        is_equation: mapped.is_equation,
    }
}


fn evaluate_side(elements: &[Element]) -> f64 {
    elements.iter().map(|el| evaluate_term(el)).sum()
}

fn evaluate_term(el: &Element) -> f64 {
    let (sign, body) = split_sign(&el.value);
    
    let magnitude = match el.element_type.as_str() {
        // P - Parentheses: recurse into the bracket
        "Bracket" => {
            let inner = body.trim_start_matches('(').trim_end_matches(')');
            let spaced = space_out_operators(inner);
            evaluate_side(&map_elements(&spaced).left_side)
        }
        
        // E - Exponents / Powers
        "Power" => eval_binary_op(body, "^", |a, b| a.powf(b)),
        "Exponent" => eval_binary_op(body, "##", |a, b| a.powf(b)),
        
        // MD - Multiplication / Division (left-to-right)
        "Multiplication" => eval_left_to_right(body, '*', |a, b| a * b),
        "Fraction" => eval_left_to_right(body, '/', |a, b| a / b),
        
        // Single number
        "Constant" => parse_number(body).unwrap_or(0.0),
        
        // Variables without numbers (like x, y) — treat as 0 for pure evaluation
        // or panic if you want strict numeric evaluation
        "Variable" => parse_number(body).unwrap_or(0.0),
        
        _ => 0.0,
    };
    
    sign * magnitude
}

/// Pulls leading + or - off a term
fn split_sign(s: &str) -> (f64, &str) {
    if s.starts_with('-') {
        (-1.0, &s[1..])
    } else if s.starts_with('+') {
        (1.0, &s[1..])
    } else {
        (1.0, s)
    }
}

/// Evaluates operators left-to-right (for * and /)
fn eval_left_to_right(body: &str, op: char, operation: fn(f64, f64) -> f64) -> f64 {
    let parts: Vec<&str> = body.split(op).collect();
    if parts.is_empty() {
        return parse_number(body).unwrap_or(0.0);
    }
    
    let mut result = parse_or_eval(parts[0]);
    for part in &parts[1..] {
        result = operation(result, parse_or_eval(part));
    }
    result
}

/// For ^ and ## (right-associative: 2^3^2 = 2^(3^2) = 512)
fn eval_binary_op(body: &str, op: &str, operation: fn(f64, f64) -> f64) -> f64 {
    let parts: Vec<&str> = body.split(op).collect();
    if parts.len() < 2 {
        return parse_or_eval(body);
    }
    
    // Right-associative: fold from the right
    let mut result = parse_or_eval(parts.last().unwrap());
    for part in parts[..parts.len()-1].iter().rev() {
        result = operation(parse_or_eval(part), result);
    }
    result
}

/// Tries to parse a number, recurses if it's a bracket or sub-expression
fn parse_or_eval(s: &str) -> f64 {
    let trimmed = s.trim();
    
    // Nested brackets like (2+3)
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        let inner = &trimmed[1..trimmed.len()-1];
        let spaced = space_out_operators(inner);
        return evaluate_side(&map_elements(&spaced).left_side);
    }
    
    parse_number(trimmed).unwrap_or_else(|_| {
        // Not a plain number — might be an expression without spaces
        let spaced = space_out_operators(trimmed);
        evaluate_side(&map_elements(&spaced).left_side)
    })
}

fn parse_number(s: &str) -> Result<f64, ()> {
    s.parse::<f64>().map_err(|_| ())
}

/// Converts "2+3*4" → "2 +3*4" so map_elements() can tokenize it
fn space_out_operators(expr: &str) -> String {
    let mut result = String::with_capacity(expr.len() * 2);
    let chars: Vec<char> = expr.chars().collect();
    
    for (i, &c) in chars.iter().enumerate() {
        // Space before binary + or - (not unary at start)
        if i > 0 && (c == '+' || c == '-') {
            let prev = chars[i - 1];
            if prev.is_ascii_digit() || prev == ')' || prev == ']' {
                result.push(' ');
            }
        }
        result.push(c);
    }
    result
}