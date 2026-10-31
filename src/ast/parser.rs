use std::println;

use crate::ast::lexer::{Token, TokenKind, lex};


#[derive(Debug)]
pub enum Expr {
    Num(String),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),              
    Call(String, Vec<Expr>),     
    Eq(Box<Expr>, Box<Expr>),    
}
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    len: usize, // length of the original text, used for "end of input" errors
}

const FUNCTIONS: &[&str] = &[
    "sin", "cos", "tan", "asin", "acos", "atan",
    "ln", "log", "exp", "sqrt", "abs",
];

fn is_function(name: &str) -> bool {
    FUNCTIONS.contains(&name)
}
impl Parser {
    fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.pos).map(|t| &t.kind)
    }

    // Position of the next token, or the end of the text. Used in error messages.
    fn here(&self) -> usize {
        self.tokens.get(self.pos).map_or(self.len, |t| t.start)
    }

    // Insist that the next token is ')' and skip it.
    fn expect_rparen(&mut self) -> Result<(), String> {
        if self.peek() == Some(&TokenKind::RParen) {
            self.pos += 1;
            Ok(())
        } else {
            Err(format!("expected ')' at position {}", self.here()))
        }

        
    }
    fn sum(&mut self) -> Result<Expr, String> {
        let mut left = self.chunk()?;
        loop {
            match self.peek() {
                Some(TokenKind::Plus) => {
                    self.pos += 1;
                    let right = self.chunk()?;
                    left = Expr::Add(Box::new(left), Box::new(right));
                }
                Some(TokenKind::Minus) => {
                    self.pos += 1;
                    let right = self.chunk()?;
                    left = Expr::Sub(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }
    fn unary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some(TokenKind::Minus) => {
                self.pos += 1;
                Ok(Expr::Neg(Box::new(self.unary()?)))
            }
            Some(TokenKind::Plus) => {
                self.pos += 1;
                self.unary()
            }
            _ => self.power(),
        }
    }
    fn power(&mut self) -> Result<Expr, String> {
        let base = self.atom()?;
        if self.peek() == Some(&TokenKind::Caret) {
            self.pos += 1;
            let exponent = self.unary()?;   // was self.power()
            Ok(Expr::Pow(Box::new(base), Box::new(exponent)))
        } else {
            Ok(base)
        }
    }
    fn atom(&mut self) -> Result<Expr, String> {
        let Some(token) = self.tokens.get(self.pos).cloned() else {
            return Err(format!("unexpected end of input at position {}", self.len));
        };
        let start = token.start;
        self.pos += 1;

        match token.kind {
            TokenKind::Num(text) => Ok(Expr::Num(text)),

            TokenKind::Ident(name) => {
                if is_function(&name) {
                    // a function name must be followed by '('
                    if self.peek() != Some(&TokenKind::LParen) {
                        return Err(format!("expected '(' after '{}' at position {}", name, self.here()));
                    }
                    self.pos += 1;

                    // read the arguments: sum, sum, sum ...
                    let mut args = vec![self.sum()?];
                    while self.peek() == Some(&TokenKind::Comma) {
                        self.pos += 1;
                        args.push(self.sum()?);
                    }
                    self.expect_rparen()?;
                    Ok(Expr::Call(name, args))
                } else {
                    Ok(Expr::Var(name))
                }
            }

            TokenKind::LParen => {
                let inner = self.sum()?;
                self.expect_rparen()?;
                Ok(inner)
            }

            _ => Err(format!("unexpected token at position {}", start)),
        }
    }

    fn chunk(&mut self) -> Result<Expr, String> {
        let mut left = self.unary()?;
        loop {
            match self.peek() {
                Some(TokenKind::Star) => {
                    self.pos += 1;
                    let right = self.unary()?;
                    left = Expr::Mul(Box::new(left), Box::new(right));
                }
                Some(TokenKind::Slash) => {
                    self.pos += 1;
                    let right = self.unary()?;
                    left = Expr::Div(Box::new(left), Box::new(right));
                }
                // 9x and 2(x+1): no symbol, but still a product
                Some(TokenKind::Ident(_)) | Some(TokenKind::LParen) => {
                    let right = self.power()?;
                    left = Expr::Mul(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

}


pub fn parse(src: &str) -> Result<Expr, String> {
    let tokens = lex(src).map_err(|e| e.to_string())?;
    let mut p = Parser { tokens, pos: 0, len: src.len() };

    let left = p.sum()?;

    let result = if p.peek() == Some(&TokenKind::Eq) {
        p.pos += 1;
        let right = p.sum()?;
        Expr::Eq(Box::new(left), Box::new(right))
    } else {
        left
    };

    // After a full parse nothing should be left over.
    if let Some(t) = p.tokens.get(p.pos) {
        return Err(format!("unexpected token at position {}", t.start));
    }
    Ok(result)
}

pub fn show(e: &Expr) -> String {
    match e {
        Expr::Num(n) => n.clone(),
        Expr::Var(v) => v.clone(),
        Expr::Add(a, b) => format!("({} + {})", show(a), show(b)),
        Expr::Sub(a, b) => format!("({} - {})", show(a), show(b)),
        Expr::Mul(a, b) => format!("({} * {})", show(a), show(b)),
        Expr::Div(a, b) => format!("({} / {})", show(a), show(b)),
        Expr::Pow(a, b) => format!("({} ^ {})", show(a), show(b)),
        Expr::Neg(a) => format!("(-{})", show(a)),
        Expr::Call(name, args) => {
            let inside: Vec<String> = args.iter().map(show).collect();
          return  format!("{}({})", name, inside.join(", "))
        }
        Expr::Eq(a, b) => format!("{} = {}", show(a), show(b)),
    }
}