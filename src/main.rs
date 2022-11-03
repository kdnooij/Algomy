use parser::parse;

use crate::simplify::simplify;

mod expression;
mod parser;
mod evaluate;
mod simplify;

extern crate pest;
#[macro_use]
extern crate pest_derive;

// Algomy
fn main() {
    /* let x = parse("(1 + 2) / 4").unwrap();
    println!("{}", x[0]);
    let y = simplify(&x[0]);
    println!("{}", y); */
}
