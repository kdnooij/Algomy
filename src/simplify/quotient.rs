use crate::expression::Expr;

use super::{power::simplify_power, product::simplify_product};

pub fn simplify_quotient(expr: &Expr) -> Expr {
    let n = &expr.operands[0];
    let d = &expr.operands[1];

    simplify_product(&Expr::times(
        n.clone(),
        simplify_power(&Expr::power(d.clone(), Expr::int(-1))),
    ))
}

// TODO: Write tests