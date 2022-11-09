use crate::{
    expression::{Expr, ExprKind},
    simplify,
};

use super::polynomial_remainder;

pub fn polynomial_gcd(u: &Expr, v: &Expr, x: &Expr) -> Expr {
    if let (ExprKind::Integer(0), ExprKind::Integer(0)) = (&u.kind, &v.kind) {
        Expr::int(0)
    } else {
        let mut a = u.clone();
        let mut b = v.clone();
        loop {
            if let ExprKind::Integer(0) = b.kind {
                return simplify(
                    &Expr::times(
                        Expr::quotient(Expr::int(1), a.leading_coefficient_gpe(x)),
                        a,
                    )
                    .algebraic_expand(),
                );
            }
            let r = simplify(&polynomial_remainder(&a, &b, x));
            a = b.clone();
            b = r;
        }
    }
}
