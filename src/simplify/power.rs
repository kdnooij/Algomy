use crate::expression::{Expr, ExprKind};

use super::{product::simplify_product, rational_number::simplify_rne};

pub fn simplify_power(u: &Expr) -> Expr {
    let v = &u.operands[0];
    let w = &u.operands[1];
    match (&v.kind, &w.kind) {
        (ExprKind::Undefined, _) | (_, ExprKind::Undefined) => Expr::undefined(),
        (ExprKind::Integer(0), _) => {
            if w.is_positive_num() {
                Expr::int(0)
            } else {
                Expr::undefined()
            }
        }
        (ExprKind::Integer(1), _) => Expr::int(1),
        (_, ExprKind::Integer(n)) => simplify_integer_power(&v, *n),
        (_, _) => u.clone(),
    }
}

pub fn simplify_integer_power(v: &Expr, n: i64) -> Expr {
    match (&v.kind, n) {
        (ExprKind::Integer(_) | ExprKind::Fraction(_, _), _) => {
            simplify_rne(&Expr::power(v.clone(), Expr::int(n)))
        }
        (_, 0) => Expr::int(1),
        (_, 1) => v.clone(),
        (ExprKind::Power, _) => {
            let r = v.operands[0].clone();
            let s = v.operands[1].clone();
            let p = simplify_product(&Expr::times(s, Expr::int(n)));
            if let ExprKind::Integer(p) = p.kind {
                simplify_integer_power(&r, p)
            } else {
                Expr::power(r, p)
            }
        }
        (ExprKind::Product, _) => {
            let r = v.map(|e| simplify_integer_power(e, n));
            simplify_product(&r)
        }
        (_, _) => Expr::power(v.clone(), Expr::int(n)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{expression::{Expr, ExprKind}, simplify::power::simplify_power};

    #[test]
    fn test_simplify_power() {
        // (((x^(1/2))^(1/2))^8 -> x^4
        assert_eq!(
            simplify_power(&Expr::power(
                Expr::power(
                    Expr::power(Expr::symbol("x"), Expr::frac(1, 2)),
                    Expr::frac(1, 2)
                ),
                Expr::int(8)
            )),
            Expr::power(Expr::symbol("x"), Expr::int(2))
        );
        // ((x*y)^(1/2)*z^2)^2 -> x*y*z^4
        assert_eq!(
            simplify_power(&Expr::power(
                Expr::times(
                    Expr::power(
                        Expr::times(Expr::symbol("x"), Expr::symbol("y")),
                        Expr::frac(1, 2)
                    ),
                    Expr::power(Expr::symbol("z"), Expr::int(2))
                ),
                Expr::int(2)
            )),
            Expr {
                kind: ExprKind::Product,
                operands: vec![
                    Expr::symbol("x"),
                    Expr::symbol("y"),
                    Expr::power(Expr::symbol("z"), Expr::int(4))
                ]
            }
        )
    }
}
