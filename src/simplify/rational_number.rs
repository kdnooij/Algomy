use num::Integer;

use crate::{
    evaluate::{
        evaluate_difference, evaluate_power, evaluate_product, evaluate_quotient, evaluate_sum,
    },
    expression::{Expr, ExprKind},
};

/// Simplify a rational number to standard form.
///
/// Takes a fraction or an integer and returns a fraction in standard form or an integer.
pub fn simplify_rational_number(u: &Expr) -> Expr {
    match u.kind {
        ExprKind::Integer(_) => u.clone(),
        ExprKind::Fraction(n, d) => {
            if n.rem_euclid(d) == 0 {
                Expr::int(n.div_euclid(d))
            } else {
                let g = n.gcd(&d);
                if d > 0 {
                    Expr::frac(n.div_euclid(g), d.div_euclid(g))
                } else {
                    Expr::frac((-n).div_euclid(g), (-d).div_euclid(g))
                }
            }
        }
        _ => unreachable!(),
    }
}

/// Simplifies a rational number expression (RNE).
///
/// Takes an RNE and outputs an integer, fraction in standard form, or undefined
pub fn simplify_rne(u: &Expr) -> Expr {
    let v = simplify_rne_recursive(u);
    if let ExprKind::Undefined = v.kind {
        Expr::undefined()
    } else {
        simplify_rational_number(&v)
    }
}

fn simplify_rne_recursive(u: &Expr) -> Expr {
    match u.kind {
        ExprKind::Integer(_) => u.clone(),
        ExprKind::Fraction(_, 0) => Expr::undefined(),
        ExprKind::Fraction(_, _) => u.clone(),
        _ => {
            if u.num_operands() == 1 {
                let v = simplify_rne_recursive(&u.operands[0]);
                if v.is_undefined() {
                    Expr::undefined()
                } else if u.kind == ExprKind::Sum {
                    v
                } else if u.kind == ExprKind::Difference {
                    evaluate_product(&Expr::int(-1), &v)
                } else {
                    unreachable!()
                }
            } else if u.num_operands() == 2 {
                match u.kind {
                    ExprKind::Sum
                    | ExprKind::Product
                    | ExprKind::Difference
                    | ExprKind::Quotient => {
                        let v = simplify_rne_recursive(&u.operands[0]);
                        let w = simplify_rne_recursive(&u.operands[1]);
                        if v.is_undefined() || w.is_undefined() {
                            Expr::undefined()
                        } else {
                            match u.kind {
                                ExprKind::Sum => evaluate_sum(&v, &w),
                                ExprKind::Difference => evaluate_difference(&v, &w),
                                ExprKind::Product => evaluate_product(&v, &w),
                                ExprKind::Quotient => evaluate_quotient(&v, &w),
                                _ => unreachable!(),
                            }
                        }
                    }
                    ExprKind::Power => {
                        let v = simplify_rne_recursive(&u.operands[0]);
                        if v.is_undefined() {
                            Expr::undefined()
                        } else {
                            // Since u is an RNE, u.operands[1] is an integer
                            evaluate_power(&v, u.operands[1].numerator())
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expression::Expr,
        simplify::rational_number::{simplify_rational_number, simplify_rne},
    };

    #[test]
    fn test_simplify_rational_number() {
        assert_eq!(
            simplify_rational_number(&Expr::frac(1, 2)),
            Expr::frac(1, 2)
        );
        assert_eq!(simplify_rational_number(&Expr::frac(6, 2)), Expr::int(3));
        assert_eq!(
            simplify_rational_number(&Expr::frac(-4, 8)),
            Expr::frac(-1, 2)
        );
        assert_eq!(
            simplify_rational_number(&Expr::frac(5, -7)),
            Expr::frac(-5, 7)
        );
        assert_eq!(
            simplify_rational_number(&Expr::frac(-5, -15)),
            Expr::frac(1, 3)
        );
    }

    #[test]
    fn test_simplify_rne() {
        assert_eq!(
            simplify_rne(&Expr::plus(Expr::frac(2, 3), Expr::frac(3, 4))),
            Expr::frac(17, 12)
        );
        assert_eq!(
            simplify_rne(&Expr::power(Expr::frac(4, 2), Expr::int(3))),
            Expr::int(8)
        );
        assert_eq!(
            simplify_rne(&Expr::quotient(
                Expr::int(1),
                Expr::minus(Expr::frac(2, 4), Expr::frac(1, 2))
            )),
            Expr::undefined()
        )
    }
}
