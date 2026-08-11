use std::collections::HashMap;


pub fn transform_equation(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() { return "0".to_string(); }

    if trimmed.contains('=') {
        let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
        let left = parse_and_simplify(parts[0].trim());
        let right = parse_and_simplify(parts[1].trim());

        // Move everything to left: left - right = 0
        let mut combined = subtract_expr(left.clone(), right.clone());
        combined = simplify_ast(combined);
        combined = combine_like_terms_ast(combined);

        // Try to solve linear equation
        if let Some(solution) = solve_linear(&combined) {
            return solution;
        }

        // Try to factor quadratic = 0
        if let Some(factored) = try_factor_quadratic(&combined) {
            return format!("{} = 0  →  {}", ast_to_string(&combined), factored);
        }

        // Standard form
        format!("{} = 0", ast_to_string(&combined))
    } else {
        let expr = parse_and_simplify(trimmed);
        let factored = try_factor_expression(&expr);
        if let Some(f) = factored {
            if f != ast_to_string(&expr) {
                return format!("{}  →  {}", ast_to_string(&expr), f);
            }
        }
        ast_to_string(&expr)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}


#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    Number(f64),
    Variable(String),
    Plus, Minus, Mul, Div, Pow,
    LParen, RParen,
    EOF,
}

pub fn tokenize(s: &str) -> Vec<Tok> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        match c {
            ' ' | '\t' | '\n' | '\r' => i += 1,
            '+' => { tokens.push(Tok::Plus); i += 1; }
            '-' => { tokens.push(Tok::Minus); i += 1; }
            '*' => { tokens.push(Tok::Mul); i += 1; }
            '/' => { tokens.push(Tok::Div); i += 1; }
            '^' => { tokens.push(Tok::Pow); i += 1; }
            '(' => { tokens.push(Tok::LParen); i += 1; }
            ')' => { tokens.push(Tok::RParen); i += 1; }
            _ if c.is_ascii_digit() || c == '.' => {
                let mut num = String::new();
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    num.push(chars[i]); i += 1;
                }
                tokens.push(Tok::Number(num.parse().unwrap_or(0.0)));
            }
            _ if c.is_alphabetic() => {
                let mut var = String::new();
                while i < chars.len() && chars[i].is_alphabetic() {
                    var.push(chars[i]); i += 1;
                }
                tokens.push(Tok::Variable(var));
            }
            _ => i += 1,
        }
    }
    tokens.push(Tok::EOF);
    tokens
}


pub struct Parser {
    tokens: Vec<Tok>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Tok>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Tok {
        self.tokens.get(self.pos).unwrap_or(&Tok::EOF)
    }

    fn advance(&mut self) -> Tok {
        let t = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 { self.pos += 1; }
        t
    }

    fn parse(&mut self) -> Expr {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_term();
        while matches!(self.peek(), Tok::Plus | Tok::Minus) {
            let op = self.advance();
            let right = self.parse_term();
            left = match op {
                Tok::Plus => Expr::Add(Box::new(left), Box::new(right)),
                Tok::Minus => Expr::Sub(Box::new(left), Box::new(right)),
                _ => left,
            };
        }
        left
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_power();
        while matches!(self.peek(), Tok::Mul | Tok::Div) {
            let op = self.advance();
            let right = self.parse_power();
            left = match op {
                Tok::Mul => Expr::Mul(Box::new(left), Box::new(right)),
                Tok::Div => Expr::Div(Box::new(left), Box::new(right)),
                _ => left,
            };
        }
        left
    }

    fn parse_power(&mut self) -> Expr {
        let mut left = self.parse_factor();
        if matches!(self.peek(), Tok::Pow) {
            self.advance();
            let right = self.parse_power(); // right-associative
            left = Expr::Pow(Box::new(left), Box::new(right));
        }
        left
    }

    fn parse_factor(&mut self) -> Expr {
        match self.peek() {
            Tok::Number(n) => {
                let val = *n;
                self.advance();
                // implicit multiplication: 2x, 2(x+1), 3y^2
                if self.is_implicit_mul_follows() {
                    let right = self.parse_factor();
                    return Expr::Mul(Box::new(Expr::Num(val)), Box::new(right));
                }
                Expr::Num(val)
            }
            Tok::Variable(v) => {
                let name = v.clone();
                self.advance();
                // implicit multiplication: x(y+1)
                if self.is_implicit_mul_follows() {
                    let right = self.parse_factor();
                    return Expr::Mul(Box::new(Expr::Var(name)), Box::new(right));
                }
                Expr::Var(name)
            }
            Tok::LParen => {
                self.advance();
                let expr = self.parse_expr();
                if matches!(self.peek(), Tok::RParen) { self.advance(); }
                // implicit multiplication: (a+b)(c+d), (a+b)3, (a+b)x
                if self.is_implicit_mul_follows() {
                    let right = self.parse_factor();
                    return Expr::Mul(Box::new(expr), Box::new(right));
                }
                expr
            }
            Tok::Minus => {
                self.advance();
                Expr::Neg(Box::new(self.parse_factor()))
            }
            _ => {
                self.advance();
                Expr::Num(0.0)
            }
        }
    }

    fn is_implicit_mul_follows(&self) -> bool {
       return matches!(self.peek(),
            Tok::Variable(_) | Tok::Number(_) | Tok::LParen)
    }
}

fn parse(s: &str) -> Expr {
    let tokens = tokenize(s);
    let mut parser = Parser::new(tokens);
    parser.parse()
}


fn parse_and_simplify(s: &str) -> Expr {
    let ast = parse(s);
    let ast = simplify_ast(ast);
    combine_like_terms_ast(ast)
}

fn simplify_ast(e: Expr) -> Expr {
    match e {
        Expr::Add(a, b) => {
            let a = simplify_ast(*a);
            let b = simplify_ast(*b);
            match (&a, &b) {
                (Expr::Num(0.0), _) => b,
                (_, Expr::Num(0.0)) => a,
                (Expr::Num(x), Expr::Num(y)) => Expr::Num(x + y),
                _ => Expr::Add(Box::new(a), Box::new(b)),
            }
        }
        Expr::Sub(a, b) => {
            let a = simplify_ast(*a);
            let b = simplify_ast(*b);
            match (&a, &b) {
                (_, Expr::Num(0.0)) => a,
                (Expr::Num(x), Expr::Num(y)) => Expr::Num(x - y),
                _ if a == b => Expr::Num(0.0),
                _ => Expr::Sub(Box::new(a), Box::new(b)),
            }
        }
        Expr::Mul(a, b) => {
            let a = simplify_ast(*a);
            let b = simplify_ast(*b);
            match (&a, &b) {
                (Expr::Num(0.0), _) | (_, Expr::Num(0.0)) => Expr::Num(0.0),
                (Expr::Num(1.0), _) => b,
                (_, Expr::Num(1.0)) => a,
                (Expr::Num(x), Expr::Num(y)) => Expr::Num(x * y),
                // Distribute: a * (b + c)
                (_, Expr::Add(c, d)) => simplify_ast(Expr::Add(
                    Box::new(Expr::Mul(Box::new(a.clone()), c.clone())),
                    Box::new(Expr::Mul(Box::new(a.clone()), d.clone())),
                )),
                (Expr::Add(c, d), _) => simplify_ast(Expr::Add(
                    Box::new(Expr::Mul(c.clone(), Box::new(b.clone()))),
                    Box::new(Expr::Mul(d.clone(), Box::new(b.clone()))),
                )),
                // Distribute: a * (b - c)
                (_, Expr::Sub(c, d)) => simplify_ast(Expr::Sub(
                    Box::new(Expr::Mul(Box::new(a.clone()), c.clone())),
                    Box::new(Expr::Mul(Box::new(a.clone()), d.clone())),
                )),
                (Expr::Sub(c, d), _) => simplify_ast(Expr::Sub(
                    Box::new(Expr::Mul(c.clone(), Box::new(b.clone()))),
                    Box::new(Expr::Mul(d.clone(), Box::new(b.clone()))),
                )),
                // Expand (a + b)^2
                (Expr::Add(c, d), Expr::Pow(e, f))
                    if **e == a && matches!(f.as_ref(), Expr::Num(2.0)) =>
                {
                    expand_binomial_sq(c, d)
                }
                (Expr::Pow(e, f), Expr::Add(c, d))
                    if **e == b && matches!(f.as_ref(), Expr::Num(2.0)) =>
                {
                    expand_binomial_sq(c, d)
                }
                _ => Expr::Mul(Box::new(a), Box::new(b)),
            }
        }
        Expr::Div(a, b) => {
            let a = simplify_ast(*a);
            let b = simplify_ast(*b);
            match (&a, &b) {
                (_, Expr::Num(1.0)) => a,
                (Expr::Num(0.0), _) => Expr::Num(0.0),
                (Expr::Num(x), Expr::Num(y)) if y != &0.0 => {
                    let r = x / y;
                    if r.fract() == 0.0 { Expr::Num(r) } else { Expr::Div(Box::new(a), Box::new(b)) }
                }
                // (2x + 4) / 2  →  x + 2
                (Expr::Add(c, d), Expr::Num(n)) if n != &0.0 => {
                    simplify_ast(Expr::Add(
                        Box::new(Expr::Div(c.clone(), Box::new(Expr::Num(*n)))),
                        Box::new(Expr::Div(d.clone(), Box::new(Expr::Num(*n)))),
                    ))
                }
                (Expr::Sub(c, d), Expr::Num(n)) if n != &0.0 => {
                    simplify_ast(Expr::Sub(
                        Box::new(Expr::Div(c.clone(), Box::new(Expr::Num(*n)))),
                        Box::new(Expr::Div(d.clone(), Box::new(Expr::Num(*n)))),
                    ))
                }
                _ => Expr::Div(Box::new(a), Box::new(b)),
            }
        }
        Expr::Pow(a, b) => {
            let a = simplify_ast(*a);
            let b = simplify_ast(*b);
            match (&a, &b) {
                (_, Expr::Num(0.0)) => Expr::Num(1.0),
                (_, Expr::Num(1.0)) => a,
                (Expr::Num(0.0), _) => Expr::Num(0.0),
                (Expr::Num(1.0), _) => Expr::Num(1.0),
                (Expr::Num(x), Expr::Num(y)) => Expr::Num(x.powf(*y)),
                // (a + b)^2
                (Expr::Add(c, d), Expr::Num(2.0)) => expand_binomial_sq(c, d),
                // (a + b)^3
                (Expr::Add(c, d), Expr::Num(3.0)) => expand_binomial_cube(c, d),
                _ => Expr::Pow(Box::new(a), Box::new(b)),
            }
        }
        Expr::Neg(a) => {
            let a = simplify_ast(*a);
            match a {
                Expr::Num(n) => Expr::Num(-n),
                Expr::Neg(b) => *b,
                _ => Expr::Neg(Box::new(a)),
            }
        }
        other => other,
    }
}

fn expand_binomial_sq(a: &Expr, b: &Expr) -> Expr {
    // a^2 + 2ab + b^2
    let a2 = Expr::Pow(Box::new(a.clone()), Box::new(Expr::Num(2.0)));
    let b2 = Expr::Pow(Box::new(b.clone()), Box::new(Expr::Num(2.0)));
    let two_ab = Expr::Mul(Box::new(Expr::Num(2.0)),
        Box::new(Expr::Mul(Box::new(a.clone()), Box::new(b.clone()))));
    simplify_ast(Expr::Add(
        Box::new(simplify_ast(a2)),
        Box::new(Expr::Add(Box::new(two_ab), Box::new(simplify_ast(b2)))),
    ))
}

fn expand_binomial_cube(a: &Expr, b: &Expr) -> Expr {
    // a^3 + 3a^2b + 3ab^2 + b^3
    let a3 = Expr::Pow(Box::new(a.clone()), Box::new(Expr::Num(3.0)));
    let b3 = Expr::Pow(Box::new(b.clone()), Box::new(Expr::Num(3.0)));
    let a2b = Expr::Mul(Box::new(Expr::Pow(Box::new(a.clone()), Box::new(Expr::Num(2.0)))),
        Box::new(b.clone()));
    let ab2 = Expr::Mul(Box::new(a.clone()),
        Box::new(Expr::Pow(Box::new(b.clone()), Box::new(Expr::Num(2.0)))));
    let t1 = Expr::Mul(Box::new(Expr::Num(3.0)), Box::new(a2b));
    let t2 = Expr::Mul(Box::new(Expr::Num(3.0)), Box::new(ab2));
    simplify_ast(Expr::Add(
        Box::new(simplify_ast(a3)),
        Box::new(Expr::Add(
            Box::new(simplify_ast(t1)),
            Box::new(Expr::Add(Box::new(simplify_ast(t2)), Box::new(simplify_ast(b3)))),
        )),
    ))
}

fn subtract_expr(a: Expr, b: Expr) -> Expr {
    Expr::Add(Box::new(a), Box::new(Expr::Neg(Box::new(b))))
}


fn combine_like_terms_ast(e: Expr) -> Expr {
    let flat = flatten_add(&e);
    let mut const_sum = 0.0;
    let mut terms: HashMap<(String, i32), f64> = HashMap::new();

    for term in &flat {
        let (coeff, var, pow) = extract_term(term);
        if var.is_empty() {
            const_sum += coeff;
        } else {
            *terms.entry((var, pow)).or_insert(0.0) += coeff;
        }
    }

    let mut result: Vec<Expr> = Vec::new();

    // Add variable terms (sorted by power descending)
    let mut var_list: Vec<((String, i32), f64)> = terms.into_iter().collect();
    var_list.sort_by(|a, b| b.0.1.cmp(&a.0.1));

    for ((var, pow), coeff) in var_list {
        if coeff == 0.0 { continue; }
        result.push(build_term(coeff, &var, pow));
    }

    if const_sum != 0.0 || result.is_empty() {
        result.push(Expr::Num(const_sum));
    }

    // Rebuild addition chain
    if result.is_empty() { return Expr::Num(0.0); }
    let mut out = result[0].clone();
    for i in 1..result.len() {
        out = Expr::Add(Box::new(out), Box::new(result[i].clone()));
    }
    out
}

fn flatten_add(e: &Expr) -> Vec<Expr> {
    match e {
        Expr::Add(a, b) => {
            let mut v = flatten_add(a);
            v.extend(flatten_add(b));
            v
        }
        Expr::Sub(a, b) => {
            let mut v = flatten_add(a);
            v.push(Expr::Neg(Box::new((**b).clone())));
            v
        }
        _ => vec![e.clone()],
    }
}

fn extract_term(e: &Expr) -> (f64, String, i32) {
    match e {
        Expr::Num(n) => (*n, "".to_string(), 0),
        Expr::Var(v) => (1.0, v.clone(), 1),
        Expr::Neg(a) => {
            let (c, v, p) = extract_term(a);
            (-c, v, p)
        }
        Expr::Mul(a, b) => {
            let (c1, v1, p1) = extract_term(a);
            let (c2, v2, p2) = extract_term(b);
            if v1.is_empty() {
                (c1 * c2, v2, p2)
            } else if v2.is_empty() {
                (c1 * c2, v1, p1)
            } else if v1 == v2 {
                (c1 * c2, v1, p1 + p2)
            } else {
                (c1 * c2, format!("{}*{}" , v1, v2), 1)
            }
        }
        Expr::Pow(a, b) => {
            if let Expr::Num(n) = b.as_ref() {
                let (_, v, _) = extract_term(a);
                (1.0, v, *n as i32)
            } else {
                (1.0, ast_to_string(e), 1)
            }
        }
        _ => (1.0, ast_to_string(e), 1),
    }
}

fn build_term(coeff: f64, var: &str, pow: i32) -> Expr {
    if var.is_empty() {
        return Expr::Num(coeff);
    }
    let var_expr = if pow == 1 {
        Expr::Var(var.to_string())
    } else {
        Expr::Pow(Box::new(Expr::Var(var.to_string())), Box::new(Expr::Num(pow as f64)))
    };
    if coeff == 1.0 {
        var_expr
    } else if coeff == -1.0 {
        Expr::Neg(Box::new(var_expr))
    } else {
        Expr::Mul(Box::new(Expr::Num(coeff)), Box::new(var_expr))
    }
}


fn solve_linear(e: &Expr) -> Option<String> {
    let flat = flatten_add(e);
    let mut coeff_x = 0.0;
    let mut constant = 0.0;

    for term in &flat {
        let (c, v, p) = extract_term(term);
        if v == "x" && p == 1 {
            coeff_x += c;
        } else if v.is_empty() {
            constant += c;
        } else {
            return None; // Not linear in x
        }
    }

    if coeff_x == 0.0 { return None; }
    let solution = -constant / coeff_x;
    let s = if solution.fract() == 0.0 {
        format!("{:.0}", solution)
    } else {
        format!("{:.4}", solution).trim_end_matches('0').trim_end_matches('.').to_string()
    };
    Some(format!("x = {}", s))
}


fn try_factor_expression(e: &Expr) -> Option<String> {
    let flat = flatten_add(e);
    let mut terms: Vec<(f64, i32)> = Vec::new(); // coeff, power of x
    let mut has_other = false;

    for term in &flat {
        let (c, v, p) = extract_term(term);
        if v == "x" {
            terms.push((c, p));
        } else if v.is_empty() {
            terms.push((c, 0));
        } else {
            has_other = true;
        }
    }

    if has_other || terms.is_empty() { return None; }
    terms.sort_by(|a, b| b.1.cmp(&a.1));

    // Check if it's a quadratic ax^2 + bx + c
    if terms.len() == 3 && terms[0].1 == 2 && terms[1].1 == 1 && terms[2].1 == 0 {
        let a = terms[0].0;
        let b = terms[1].0;
        let c = terms[2].0;
        return factor_quadratic(a, b, c);
    }

    // Check if it's x^2 + bx (no constant)
    if terms.len() == 2 && terms[0].1 == 2 && terms[1].1 == 1 {
        let b = terms[1].0;
        if b == 1.0 { return Some("x(x + 1)".to_string()); }
        if b.fract() == 0.0 { return Some(format!("x(x + {:.0})", b)); }
        return Some(format!("x(x + {})", b));
    }

    None
}

fn try_factor_quadratic(e: &Expr) -> Option<String> {
    try_factor_expression(e)
}

fn factor_quadratic(a: f64, b: f64, c: f64) -> Option<String> {
    if a == 0.0 { return None; }
    // Find two numbers m, n such that m*n = a*c and m+n = b
    let target = a * c;
    let step = if target.fract() == 0.0 && b.fract() == 0.0 { 1.0 } else { 0.1 };
    let limit = target.abs().max(1000.0);

    let mut m = -limit;
    while m <= limit {
        if m != 0.0 && (target / m).fract() == 0.0 {
            let n = target / m;
            if (m + n - b).abs() < 1e-6 {
                // Factor by grouping
                let g1 = gcd_f64(a, m);
                let g2 = gcd_f64(n, c);
                let f1 = a / g1;
                let f2 = m / g1;
                let f3 = n / g2;
                let f4 = c / g2;

                if f1 == f3 && f2 == f4 {
                    let inner = format_term(f1, f2);
                    return Some(format!("({})^2", inner));
                }

                let group1 = format_group(a, m);
                let group2 = format_group(n, c);
                if group1 == group2 {
                    return Some(format!("({})({})", group1, format_group(g1, g2)));
                }
            }
        }
        m += step;
    }

    // Try quadratic formula for nice roots
    let disc = b * b - 4.0 * a * c;
    if disc >= 0.0 {
        let sqrt_d = disc.sqrt();
        let r1 = (-b + sqrt_d) / (2.0 * a);
        let r2 = (-b - sqrt_d) / (2.0 * a);
        if r1.fract() == 0.0 && r2.fract() == 0.0 {
            let s1 = if r1 >= 0.0 { format!("-{:.0}", r1) } else { format!("+{:.0}", -r1) };
            let s2 = if r2 >= 0.0 { format!("-{:.0}", r2) } else { format!("+{:.0}", -r2) };
            return Some(format!("(x {})(x {})", s1, s2));
        }
    }

    None
}

fn format_term(a: f64, b: f64) -> String {
    if a == 1.0 && b == 1.0 { "x + 1".to_string() }
    else if a == 1.0 { return format!("x + {:.0}", b) }
    else if b == 1.0 { return format!("{:.0}x + 1", a) }
    else { return format!("{:.0}x + {:.0}", a, b) }
}

fn format_group(a: f64, b: f64) -> String {
    let mut parts = Vec::new();
    if a != 0.0 {
        if a == 1.0 { parts.push("x".to_string()); }
        else if a == -1.0 { parts.push("-x".to_string()); }
        else { parts.push(format!("{:.0}x", a)); }
    }
    if b != 0.0 {
        if b > 0.0 && !parts.is_empty() { parts.push(format!("+ {:.0}", b)); }
        else { parts.push(format!("{:.0}", b)); }
    }
    if parts.is_empty() { "0".to_string() } else { parts.join(" ") }
}

fn gcd_f64(a: f64, b: f64) -> f64 {
    let mut a = a.abs() as i64;
    let mut b = b.abs() as i64;
    if a == 0 { return b as f64; }
    if b == 0 { return a as f64; }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a as f64
}


fn ast_to_string(e: &Expr) -> String {
    match e {
        Expr::Num(n) => {
            if n.fract() == 0.0 { format!("{:.0}", n) }
            else { return format!("{:.4}", n).trim_end_matches('0').trim_end_matches('.').to_string() }
        }
        Expr::Var(v) => v.clone(),
        Expr::Add(a, b) => format!("{} + {}", ast_to_string(a), ast_to_string(b)),
        Expr::Sub(a, b) => format!("{} - {}", ast_to_string(a), ast_to_string(b)),
        Expr::Mul(a, b) => {
            let left = ast_to_string(a);
            let right = ast_to_string(b);
            // Don't show * between number and variable
            if matches!(a.as_ref(), Expr::Num(_)) && matches!(b.as_ref(), Expr::Var(_) | Expr::Pow(_, _)) {
               return  format!("{}{}", left, right)
            } else {
              return  format!("{} * {}", left, right)
            }
        }
        Expr::Div(a, b) => format!("({}) / ({})", ast_to_string(a), ast_to_string(b)),
        Expr::Pow(a, b) => {
            let base = ast_to_string(a);
            let exp = ast_to_string(b);
            if base.len() == 1 || matches!(a.as_ref(), Expr::Var(_)) {
                return format!("{}^{}", base, exp)
            } else {
               return  format!("({})^{}", base, exp)
            }
        }
        Expr::Neg(a) => format!("-{}", ast_to_string(a)),
    }
}