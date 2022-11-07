use crate::{
    evaluate::{
        evaluate_difference_gaussian, evaluate_power_gaussian, evaluate_product,
        evaluate_product_gaussian, evaluate_quotient_gaussian, evaluate_sum_gaussian,
    },
    expression::{Expr, ExprKind},
};

use super::rational_number::simplify_rational_number;

pub fn simplify_gaussian_number(u: &Expr) -> Expr {
    match u.kind {
        ExprKind::Integer(_) => u.clone(),
        ExprKind::Fraction(_, _) => simplify_rational_number(u),
        ExprKind::Gaussian => {
            let r = simplify_rational_number(&u.operands[0]);
            let i = simplify_rational_number(&u.operands[1]);
            match (&r.kind, &i.kind) {
                // r + 0i -> r
                (_, ExprKind::Integer(0)) => r.clone(),
                _ => Expr::gaussian(r, i),
            }
        }
        _ => u.clone(),
    }
}

pub fn simplify_grne(u: &Expr) -> Expr {
    let v = simplify_grne_recursive(u);
    if let ExprKind::Undefined = v.kind {
        Expr::undefined()
    } else {
        simplify_gaussian_number(&v)
    }
}

fn simplify_grne_recursive(u: &Expr) -> Expr {
    match u.kind {
        ExprKind::Integer(_) => u.clone(),
        ExprKind::Fraction(_, 0) => Expr::undefined(),
        ExprKind::Fraction(_, _) => u.clone(),
        ExprKind::Gaussian => u.clone(),
        _ => {
            if u.num_operands() == 1 {
                let v = simplify_grne_recursive(&u.operands[0]);
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
                        let v = simplify_grne_recursive(&u.operands[0]);
                        let w = simplify_grne_recursive(&u.operands[1]);
                        if v.is_undefined() || w.is_undefined() {
                            Expr::undefined()
                        } else {
                            match u.kind {
                                ExprKind::Sum => evaluate_sum_gaussian(&v, &w),
                                ExprKind::Difference => evaluate_difference_gaussian(&v, &w),
                                ExprKind::Product => evaluate_product_gaussian(&v, &w),
                                ExprKind::Quotient => evaluate_quotient_gaussian(&v, &w),
                                _ => unreachable!(),
                            }
                        }
                    }
                    ExprKind::Power => {
                        let v = simplify_grne_recursive(&u.operands[0]);
                        if v.is_undefined() {
                            Expr::undefined()
                        } else {
                            // Since u is an RNE, u.operands[1] is an integer
                            evaluate_power_gaussian(&v, u.operands[1].numerator_rne())
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
