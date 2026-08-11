use crate::utils::element_seperator::{map_elements, Element};

pub fn calculate_bodmas(equation: &str) -> f64 {
    let mapped = map_elements(equation);
    
    if mapped.is_equation {
        panic!("calculate_bodmas() expects an expression, not an equation. Remove the = sign.");
    }
    
    evaluate_elements(&mapped.left_side)
}

fn evaluate_elements(elements: &[Element]) -> f64 {
    elements.iter().map(|el| evaluate_term(&el.value, &el.element_type)).sum()
}

fn evaluate_term(value: &str, el_type: &str) -> f64 {
    // Pull off the leading + or - sign
    let (sign, body) = if value.starts_with('-') {
        (-1.0, &value[1..])
    } else if value.starts_with('+') {
        (1.0, &value[1..])
    } else {
        (1.0, value)
    };

    let magnitude = match el_type {
        "Constant" => parse_or_eval(body),

        "Multiplication" => {
            body.split('*').map(parse_or_eval).product()
        }

        "Fraction" => {
            let parts: Vec<&str> = body.split('/').collect();
            if parts.len() == 2 {
                parse_or_eval(parts[0]) / parse_or_eval(parts[1])
            } else {
                parse_or_eval(body)
            }
        }

        "Power" => {
            let parts: Vec<&str> = body.split('^').collect();
            if parts.len() == 2 {
                parse_or_eval(parts[0]).powf(parse_or_eval(parts[1]))
            } else {
                parse_or_eval(body)
            }
        }

        "Exponent" => {
            let parts: Vec<&str> = body.split("##").collect();
            if parts.len() == 2 {
                parse_or_eval(parts[0]).powf(parse_or_eval(parts[1]))
            } else {
                parse_or_eval(body)
            }
        }

        "Bracket" => {
            let inner = body.trim_start_matches('(').trim_end_matches(')');
            let spaced = space_out_expression(inner);
            calculate_bodmas(&spaced)
        }

        _ => 0.0,
    };

    sign * magnitude
}

/// Tries to parse a number, falls back to recursive BODMAS if it's a bracket or sub-expression
fn parse_or_eval(s: &str) -> f64 {
    let trimmed = s.trim();
    
    // If it's wrapped in brackets, recurse
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        let inner = &trimmed[1..trimmed.len()-1];
        let spaced = space_out_expression(inner);
        return calculate_bodmas(&spaced);
    }
    
    // Try direct number parse
    trimmed.parse::<f64>().unwrap_or_else(|_| {
        // Not a plain number — treat as expression and recurse
        let spaced = space_out_expression(trimmed);
        calculate_bodmas(&spaced)
    })
}

/// Converts bracket innards like "4-8" or "2*3+5" into your spaced format "4 -8", "2*3 +5"
/// so map_elements() can tokenize it correctly
fn space_out_expression(expr: &str) -> String {
    let mut result = String::with_capacity(expr.len() + 4);
    let chars: Vec<char> = expr.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        // Insert space before binary + or - (not at start, and only after a number or closing bracket)
        if i > 0 && (c == '+' || c == '-') {
            let prev = chars[i - 1];
            if prev.is_ascii_digit() || prev == ')' || prev == ']' {
                result.push(' ');
                result.push(c);
                continue;
            }
        }
        result.push(c);
    }

    result
}