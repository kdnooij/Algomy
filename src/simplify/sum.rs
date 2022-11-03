use crate::expression::{Expr, ExprKind};

use super::{product::simplify_product, rational_number::simplify_rne, complex_number::simplify_cne};

pub fn simplify_sum(u: &Expr) -> Expr {
    if u.operands.iter().find(|v| v.is_undefined()).is_some() {
        // ... + undefined -> undefined
        Expr::undefined()
    } else if u.operands.len() == 1 {
        // +a -> a
        u.operands[0].clone()
    } else {
        let v: Vec<Expr> = simplify_sum_recursive(&u.operands);
        match v.len() {
            0 => Expr::int(0),
            1 => v[0].clone(),
            _ => Expr {
                kind: ExprKind::Sum,
                operands: v,
            },
        }
    }
}

pub fn simplify_sum_recursive(l: &[Expr]) -> Vec<Expr> {
    if l.len() == 2 && l[0].kind != ExprKind::Sum && l[1].kind != ExprKind::Sum {
        let u1 = &l[0];
        let u2 = &l[1];
        match (&u1.kind, &u2.kind) {
            (
                ExprKind::Integer(_) | ExprKind::Fraction(_, _) | ExprKind::Complex,
                ExprKind::Integer(_) | ExprKind::Fraction(_, _) | ExprKind::Complex,
            ) => {
                // addition of constant operands in sum
                let p = simplify_cne(&Expr::plus(u1.clone(), u2.clone()));
                if let ExprKind::Integer(0) = p.kind {
                    vec![]
                } else {
                    vec![p]
                }
            }
            // a + 0 -> a
            (ExprKind::Integer(0), _) => {
                vec![u2.clone()]
            }
            (_, ExprKind::Integer(0)) => {
                vec![u1.clone()]
            }
            _ => {
                let r1 = u1.product_rest();
                let r2 = u2.product_rest();
                if r1 == r2 {
                    // a*c + b*c -> (a+b)*c
                    let s = simplify_sum(&Expr::plus(u1.product_coeff(), u2.product_coeff()));
                    let p = simplify_product(&Expr::times(s, r1));
                    if let ExprKind::Integer(0) = p.kind {
                        vec![]
                    } else {
                        vec![p]
                    }
                } else if u2 < u1 {
                    // b + a -> a + b
                    vec![u2.clone(), u1.clone()]
                } else {
                    l.to_vec()
                }
            }
        }
    } else if l.len() == 2 && (l[0].kind == ExprKind::Sum || l[1].kind == ExprKind::Sum) {
        let u1 = &l[0];
        let u2 = &l[1];
        match [&u1.kind, &u2.kind] {
            // (a + b) + (c + d) -> a + b + c + d
            [ExprKind::Sum, ExprKind::Sum] => merge_sums(&u1.operands, &u2.operands),
            // (a + b) + c -> a + b + c
            [ExprKind::Sum, _] => merge_sums(&u1.operands, &[u2.clone()]),
            // a + (b + c) -> a + b + c
            [_, ExprKind::Sum] => merge_sums(&[u1.clone()], &u2.operands),
            _ => unreachable!(),
        }
    } else {
        // l.len() > 2
        let w = simplify_sum_recursive(&l[1..]);
        if let ExprKind::Sum = l[0].kind {
            merge_sums(&l[0].operands, &w)
        } else {
            merge_sums(&[l[0].clone()], &w)
        }
    }
}

fn merge_sums(p: &[Expr], q: &[Expr]) -> Vec<Expr> {
    if q.is_empty() {
        p.to_vec()
    } else if p.is_empty() {
        q.to_vec()
    } else {
        let p1 = &p[0];
        let q1 = &q[0];
        let h = simplify_sum_recursive(&vec![p1.clone(), q1.clone()]);
        match &h[..] {
            [] => merge_sums(&p[1..], &q[1..]),
            [h1] => {
                let mut r = vec![h1.clone()];
                r.append(&mut merge_sums(&p[1..], &q[1..]));
                r
            }
            [a, b] => {
                if a == p1 && b == q1 {
                    let mut r = vec![p1.clone()];
                    r.append(&mut merge_sums(&p[1..], &q));
                    r
                } else if a == q1 && b == p1 {
                    let mut r = vec![q1.clone()];
                    r.append(&mut merge_sums(&p, &q[1..]));
                    r
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::{Expr, ExprKind};

    use super::simplify_sum;

    #[test]
    fn test_simplify_sum() {
        // (-1)*a + a -> 0
        assert_eq!(
            simplify_sum(&Expr {
                kind: ExprKind::Sum,
                operands: vec![
                    Expr::times(Expr::int(-1), Expr::symbol("a")),
                    Expr::symbol("a"),
                ]
            }),
            Expr::int(0)
        );
        // (-1)*a + b + a -> b
        assert_eq!(
            simplify_sum(&Expr {
                kind: ExprKind::Sum,
                operands: vec![
                    Expr::times(Expr::int(-1), Expr::symbol("a")),
                    Expr::symbol("b"),
                    Expr::symbol("a"),
                ]
            }),
            Expr::symbol("b")
        );
        // c + 2 + b + c + a -> 2 + a + b + 2*c
        assert_eq!(
            simplify_sum(&Expr {
                kind: ExprKind::Sum,
                operands: vec![
                    Expr::symbol("c"),
                    Expr::int(2),
                    Expr::symbol("b"),
                    Expr::symbol("c"),
                    Expr::symbol("a"),
                ]
            }),
            simplify_sum(&Expr {
                kind: ExprKind::Sum,
                operands: vec![
                    Expr::int(2),
                    Expr::symbol("a"),
                    Expr::symbol("b"),
                    Expr::times(Expr::int(2), Expr::symbol("c")),
                ]
            }),
        );
        // (2 + a + c + e) + (3 + b + d + e) -> 5 + a + b + c + d + 2*e
        assert_eq!(
            simplify_sum(&Expr::times(
                Expr {
                    kind: ExprKind::Sum,
                    operands: vec![
                        Expr::int(2),
                        Expr::symbol("a"),
                        Expr::symbol("c"),
                        Expr::symbol("e"),
                    ]
                },
                Expr {
                    kind: ExprKind::Sum,
                    operands: vec![
                        Expr::int(3),
                        Expr::symbol("b"),
                        Expr::symbol("d"),
                        Expr::symbol("e"),
                    ]
                }
            )),
            Expr {
                kind: ExprKind::Sum,
                operands: vec![
                    Expr::int(5),
                    Expr::symbol("a"),
                    Expr::symbol("b"),
                    Expr::symbol("c"),
                    Expr::symbol("d"),
                    Expr::times(Expr::int(2), Expr::symbol("e")),
                ]
            }
        )
        // TODO: add more tests!
    }
}
