use crate::expression::{Expr, ExprKind};

use super::{
    gaussian_number::simplify_grne, power::simplify_power,
    sum::simplify_sum, merge_nary,
};

pub fn simplify_product(u: &Expr) -> Expr {
    if u.operands.iter().find(|v| v.is_undefined()).is_some() {
        // ... * undefined -> undefined
        Expr::undefined()
    } else if u
        .operands
        .iter()
        .find(|v| v.kind == ExprKind::Integer(0))
        .is_some()
    {
        // ... * 0 -> 0
        Expr::int(0)
    } else if u.operands.len() == 1 {
        // *a -> a
        u.operands[0].clone()
    } else {
        let v: Vec<Expr> = simplify_product_recursive(&u.operands);
        match v.len() {
            0 => Expr::int(1),
            1 => v[0].clone(),
            _ => Expr {
                kind: ExprKind::Product,
                operands: v,
            },
        }
    }
}

fn simplify_product_recursive(l: &[Expr]) -> Vec<Expr> {
    if l.len() == 2 && l[0].kind != ExprKind::Product && l[1].kind != ExprKind::Product {
        let u1 = &l[0];
        let u2 = &l[1];
        match (&u1.kind, &u2.kind) {
            (
                ExprKind::Integer(_) | ExprKind::Fraction(_, _) | ExprKind::Gaussian,
                ExprKind::Integer(_) | ExprKind::Fraction(_, _) | ExprKind::Gaussian,
            ) => {
                let p = simplify_grne(&Expr::times(u1.clone(), u2.clone()));
                if let ExprKind::Integer(1) = p.kind {
                    vec![]
                } else {
                    vec![p]
                }
            }
            (ExprKind::Integer(1), _) => {
                vec![u2.clone()]
            }
            (_, ExprKind::Integer(1)) => {
                vec![u1.clone()]
            }
            _ => {
                if u1.base() == u2.base() {
                    let s = simplify_sum(&Expr::plus(u1.exponent(), u2.exponent()));
                    let p = simplify_power(&Expr::power(u1.base(), s));
                    if let ExprKind::Integer(1) = p.kind {
                        vec![]
                    } else {
                        vec![p]
                    }
                } else if u2 < u1 {
                    vec![u2.clone(), u1.clone()]
                } else {
                    l.to_vec()
                }
            }
        }
    } else if l.len() == 2 && (l[0].kind == ExprKind::Product || l[1].kind == ExprKind::Product) {
        let u1 = &l[0];
        let u2 = &l[1];
        match (&u1.kind, &u2.kind) {
            (ExprKind::Product, ExprKind::Product) => merge_nary(&u1.operands, &u2.operands, simplify_product_recursive),
            (ExprKind::Product, _) => merge_nary(&u1.operands, &[u2.clone()], simplify_product_recursive),
            (_, ExprKind::Product) => merge_nary(&[u1.clone()], &u2.operands, simplify_product_recursive),
            _ => unreachable!(),
        }
    } else {
        // l.len() > 2
        let w = simplify_product_recursive(&l[1..]);
        if let ExprKind::Product = l[0].kind {
            merge_nary(&l[0].operands, &w, simplify_product_recursive)
        } else {
            merge_nary(&[l[0].clone()], &w, simplify_product_recursive)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::{Expr, ExprKind};

    use super::simplify_product;

    #[test]
    fn test_simplify_product() {
        // a^(-1) * a -> 1
        assert_eq!(
            simplify_product(&Expr {
                kind: ExprKind::Product,
                operands: vec![
                    Expr::power(Expr::symbol("a"), Expr::int(-1)),
                    Expr::symbol("a"),
                ]
            }),
            Expr::int(1)
        );
        // a^(-1) * b * a -> b
        assert_eq!(
            simplify_product(&Expr {
                kind: ExprKind::Product,
                operands: vec![
                    Expr::power(Expr::symbol("a"), Expr::int(-1)),
                    Expr::symbol("b"),
                    Expr::symbol("a"),
                ]
            }),
            Expr::symbol("b")
        );
        // c * 2 * b * c * a -> 2 * a * b * c^2
        assert_eq!(
            simplify_product(&Expr {
                kind: ExprKind::Product,
                operands: vec![
                    Expr::symbol("c"),
                    Expr::int(2),
                    Expr::symbol("b"),
                    Expr::symbol("c"),
                    Expr::symbol("a"),
                ]
            }),
            simplify_product(&Expr {
                kind: ExprKind::Product,
                operands: vec![
                    Expr::int(2),
                    Expr::symbol("a"),
                    Expr::symbol("b"),
                    Expr::power(Expr::symbol("c"), Expr::int(2)),
                ]
            }),
        );
        // (2 * a * c * e) * (3 * b * d * e) -> 6 * a * b * c * d * e^2
        assert_eq!(
            simplify_product(&Expr::times(
                Expr {
                    kind: ExprKind::Product,
                    operands: vec![
                        Expr::int(2),
                        Expr::symbol("a"),
                        Expr::symbol("c"),
                        Expr::symbol("e"),
                    ]
                },
                Expr {
                    kind: ExprKind::Product,
                    operands: vec![
                        Expr::int(3),
                        Expr::symbol("b"),
                        Expr::symbol("d"),
                        Expr::symbol("e"),
                    ]
                }
            )),
            Expr {
                kind: ExprKind::Product,
                operands: vec![
                    Expr::int(6),
                    Expr::symbol("a"),
                    Expr::symbol("b"),
                    Expr::symbol("c"),
                    Expr::symbol("d"),
                    Expr::power(Expr::symbol("e"), Expr::int(2)),
                ]
            }
        )
        // TODO: add more tests!
    }
}
