use crate::ast::parser::{parse , show};
mod special;


mod ast;
fn main() {
    let inputs = [
        "2 + 3 * 4",
        "10 - 3 - 2",
        "8 / 2 / 2",
        "-x^2",
        "2^-x",
        "2^3^2",
        "9x^2 - 4",
        "2(x + 1)",
        "sin(x)^2 + cos(x)^2",
        "log(x, 2)",
        "x^2 = 4",
        "(9x - 5^x)/(4x + 8)^x + e^x",
    ];
    for s in inputs {
        match parse(s) {
            Ok(tree) => println!("{:<32} =>  {}", s, show(&tree)),
            Err(msg) => println!("{:<32} =>  ERROR: {}", s, msg),
        }
    }


    for s in ["2 +", "(x", "x)", "2 3", "sin x", "x = = 3", "ln()", "2 $ 3"] {
        println!("{:<32} =>  {:?}", s, parse(s).map(|t| show(&t)));
    }
}