use crate::{
    evaluate::{
        evaluate_difference_complex, evaluate_power_complex, evaluate_product,
        evaluate_product_complex, evaluate_quotient_complex, evaluate_sum_complex,
    },
    expression::{Expr, ExprKind},
};

use super::rational_number::simplify_rational_number;

pub fn simplify_complex_number(u: &Expr) -> Expr {
    match u.kind {
        ExprKind::Integer(_) => u.clone(),
        ExprKind::Fraction(_, _) => simplify_rational_number(u),
        ExprKind::Complex => {
            let r = simplify_rational_number(&u.operands[0]);
            let i = simplify_rational_number(&u.operands[1]);
            match (&r.kind, &i.kind) {
                // r + 0i -> r
                (_, ExprKind::Integer(0)) => r.clone(),
                _ => Expr::complex(r, i),
            }
        }
        _ => u.clone(),
    }
}

pub fn simplify_cne(u: &Expr) -> Expr {
    let v = simplify_cne_recursive(u);
    if let ExprKind::Undefined = v.kind {
        Expr::undefined()
    } else {
        simplify_complex_number(&v)
    }
}

fn simplify_cne_recursive(u: &Expr) -> Expr {
    match u.kind {
        ExprKind::Integer(_) => u.clone(),
        ExprKind::Fraction(_, 0) => Expr::undefined(),
        ExprKind::Fraction(_, _) => u.clone(),
        ExprKind::Complex => u.clone(),
        _ => {
            if u.num_operands() == 1 {
                let v = simplify_cne_recursive(&u.operands[0]);
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
                        let v = simplify_cne_recursive(&u.operands[0]);
                        let w = simplify_cne_recursive(&u.operands[1]);
                        if v.is_undefined() || w.is_undefined() {
                            Expr::undefined()
                        } else {
                            match u.kind {
                                ExprKind::Sum => evaluate_sum_complex(&v, &w),
                                ExprKind::Difference => evaluate_difference_complex(&v, &w),
                                ExprKind::Product => evaluate_product_complex(&v, &w),
                                ExprKind::Quotient => evaluate_quotient_complex(&v, &w),
                                _ => unreachable!(),
                            }
                        }
                    }
                    ExprKind::Power => {
                        let v = simplify_cne_recursive(&u.operands[0]);
                        if v.is_undefined() {
                            Expr::undefined()
                        } else {
                            // Since u is an RNE, u.operands[1] is an integer
                            evaluate_power_complex(&v, u.operands[1].numerator())
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
