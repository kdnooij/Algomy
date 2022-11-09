use crate::expression::{Expr, ExprKind};

use super::polynomial_division;

pub fn polynomial_expansion(u: &Expr, v: &Expr, x: &Expr, t: &Expr) -> Expr {
    if let ExprKind::Integer(0) = u.kind {
        u.clone()
    } else {
        let (q, r) = polynomial_division(u, v, x);
        Expr::plus(Expr::times(t.clone(), polynomial_expansion(&q, v, x, t)), r).algebraic_expand()
    }
}
