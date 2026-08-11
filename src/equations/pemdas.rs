use crate::utils::{element_seperator::map_elements, transform_equation::{Tok, tokenize}};

pub struct PemdasResult {
    pub left: f64,
    pub right: Option<f64>,
    pub is_equation: bool,
}


pub fn calculate_pemdas(equation: &str) -> PemdasResult {
    let trimmed = equation.trim();
    let is_eq = map_elements(equation).is_equation;

    if is_eq {
        let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
        PemdasResult {
            left: eval_expression(parts[0].trim()),
            right: Some(eval_expression(parts[1].trim())),
            is_equation: true,
        }
    } else {
        PemdasResult {
            left: eval_expression(trimmed),
            right: None,
            is_equation: false,
        }
    }
}



struct Evaluator {
    tokens: Vec<Tok>,
    pos: usize,
}

impl Evaluator {
    fn new(tokens: Vec<Tok>) -> Self {
        Evaluator { tokens, pos: 0 }
    }

    fn peek(&self) -> &Tok {
        self.tokens.get(self.pos).unwrap_or(&Tok::EOF)
    }

    fn advance(&mut self) -> Tok {
        let t = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 { self.pos += 1; }
        t
    }

    fn eval(&mut self) -> f64 {
        self.parse_expr()
    }

    // expr = term { (+|-) term }
    fn parse_expr(&mut self) -> f64 {
        let mut left = self.parse_term();
        while matches!(self.peek(), Tok::Plus | Tok::Minus) {
            match self.advance() {
                Tok::Plus => left += self.parse_term(),
                Tok::Minus => left -= self.parse_term(),
                _ => break,
            }
        }
        left
    }

    // term = power { (*|/) power }   (left-to-right)
    fn parse_term(&mut self) -> f64 {
        let mut left = self.parse_power();
        while matches!(self.peek(), Tok::Mul | Tok::Div) {
            match self.advance() {
                Tok::Mul => left *= self.parse_power(),
                Tok::Div => left /= self.parse_power(),
                _ => break,
            }
        }
        left
    }

    // power = unary [ ^ power ]   (right-associative: 2^3^2 = 2^(3^2) = 512)
    fn parse_power(&mut self) -> f64 {
        let left = self.parse_unary();
        if matches!(self.peek(), Tok    ::Pow) {
            self.advance();
            let right = self.parse_power(); // right-assoc!
            left.powf(right)
        } else {
            left
        }
    }

    // unary = (-) unary | factor
    fn parse_unary(&mut self) -> f64 {
        if matches!(self.peek(), Tok::Minus) {
            self.advance();
            -self.parse_unary()
        } else if matches!(self.peek(), Tok::Plus) {
            self.advance();
            self.parse_unary()
        } else {
            self.parse_factor()
        }
    }

    // factor = number | ( expr ) | implicit_mul
    fn parse_factor(&mut self) -> f64 {
        match self.peek() {
            Tok::Number(n) => {
                let val = *n;
                self.advance();
                // Implicit multiplication: 2(3+4) or 2x (x becomes 0)
                if self.is_factor_next() { val * self.parse_factor() } else { val }
            }
            Tok::LParen => {
                self.advance();
                let val = self.parse_expr();
                if matches!(self.peek(), Tok::RParen) { self.advance(); }
                // Implicit multiplication: (2+3)(4+5) or (2+3)4
                if self.is_factor_next() { val * self.parse_factor() } else { val }
            }
            _ => {
                self.advance();
                0.0
            }
        }
    }

    fn is_factor_next(&self) -> bool {
       return matches!(self.peek(), Tok::Number(_) | Tok::LParen)
    }
}

fn eval_expression(expr: &str) -> f64 {
    let tokens = tokenize(expr);
    let mut ev = Evaluator::new(tokens);
    ev.eval()
}