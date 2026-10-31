use crate::ast::Expr;

pub fn canon(e: &Expr) -> Expr {
    match e {
        Expr::Add(a, b) => {
            let a = canon(a);
            let b = canon(b);
            match (&a, &b) {
                (Expr::Num(x), Expr::Num(y)) => Expr::Num(x + y), // 2 + 3 → 5
                (Expr::Num(x), _) if *x == 0.0 => b,              // 0 + b → b
                (_, Expr::Num(y)) if *y == 0.0 => a,              // a + 0 → a
                _ => Expr::Add(Box::new(a), Box::new(b)),
            }
        }

        Expr::Mul(a, b) => {
            let a = canon(a);
            let b = canon(b);
            match (&a, &b) {
                (Expr::Num(x), Expr::Num(y)) => Expr::Num(x * y), // 2 * 3 → 6
                (Expr::Num(x), _) if *x == 0.0 => Expr::Num(0.0), // 0 * b → 0
                (_, Expr::Num(y)) if *y == 0.0 => Expr::Num(0.0), // a * 0 → 0
                (Expr::Num(x), _) if *x == 1.0 => b,              // 1 * b → b
                (_, Expr::Num(y)) if *y == 1.0 => a,              // a * 1 → a
                _ => Expr::Mul(Box::new(a), Box::new(b)),
            }
        }

        Expr::Pow(a, b) => {
            let a = canon(a);
            let b = canon(b);
            match (&a, &b) {
                (Expr::Num(x), Expr::Num(y)) => Expr::Num(x.powf(*y)), // 2^3 → 8
                (_, Expr::Num(y)) if *y == 1.0 => a,                   // a^1 → a
                (_, Expr::Num(y)) if *y == 0.0 => Expr::Num(1.0),      // a^0 → 1
                _ => Expr::Pow(Box::new(a), Box::new(b)),
            }
        }

        // other node kinds: just tidy the insides for now
        Expr::Sub(a, b) => Expr::Sub(Box::new(canon(a)), Box::new(canon(b))),
        Expr::Div(a, b) => Expr::Div(Box::new(canon(a)), Box::new(canon(b))),
        Expr::Neg(a) => Expr::Neg(Box::new(canon(a))),
        Expr::Call(name, args) => Expr::Call(name.clone(), args.iter().map(canon).collect()),
        Expr::Eq(a, b) => Expr::Eq(Box::new(canon(a)), Box::new(canon(b))),

        // numbers and variables are already as tidy as they get
        Expr::Num(_) | Expr::Var(_) => e.clone(),
    }
}