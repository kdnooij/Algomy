use crate::expression::{Expr, ExprKind};

use super::{product::simplify_product, sum::simplify_sum};

pub fn simplify_difference(expr: &Expr) -> Expr {
    let u = &expr.operands[0];
    let v = &expr.operands[1];
    match (&u.kind, &v.kind) {
        (ExprKind::Integer(0), _) => simplify_product(&Expr::times(Expr::int(-1), v.clone())),
        _ => simplify_sum(&Expr::plus(
            u.clone(),
            // Not sure if this simplify is necessary
            simplify_product(&Expr::times(Expr::int(-1), v.clone())),
        )),
    }
}

// TODO: Write tests