use crate::{
    expression::{Expr, ExprKind},
    polynomial::{self, polynomial_quotient, polynomial_remainder},
    simplify,
};

pub fn evaluate_function(expr: &Expr) -> Expr {
    if let ExprKind::Func(ref name) = expr.kind {
        if expr
            .operands
            .iter()
            .find(|u| u.kind == ExprKind::Undefined)
            .is_none()
        {
            evaluate_function_by_name(name.as_str(), expr)
        } else {
            Expr::undefined()
        }
    } else {
        expr.clone()
    }
}

fn evaluate_function_by_name(name: &str, expr: &Expr) -> Expr {
    match (name, expr.operands.len()) {
        ("Numerator", 1) => expr.operands[0].numerator(),
        ("Denominator", 1) => expr.operands[0].denominator(),
        ("Re", 1) => expr.operands[0].re(),
        ("Im", 1) => expr.operands[0].im(),
        ("Expand", 1) => expr.operands[0].algebraic_expand(),
        ("Coefficient", 3) => {
            if let ExprKind::Integer(n) = expr.operands[2].kind {
                expr.operands[0].coefficient_gpe(&expr.operands[1], n)
            } else {
                Expr::undefined()
            }
        }
        ("PolynomialQuotient", 3) => {
            polynomial_quotient(&expr.operands[0], &expr.operands[1], &expr.operands[2])
        }
        ("PolynomialRemainder", 3) => {
            polynomial_remainder(&expr.operands[0], &expr.operands[1], &expr.operands[2])
        }
        _ => expr.clone(),
    }
}
