use crate::{
    expression::{Expr, ExprKind},
    polynomial::{polynomial_expansion, polynomial_quotient, polynomial_remainder, polynomial_gcd},
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
        ("IsInteger", 1) => Expr::bool(expr.operands[0].is_integer()),
        ("IsRNE", 1) => Expr::bool(expr.operands[0].is_rne()),
        ("IsGRNE", 1) => Expr::bool(expr.operands[0].is_grne()),
        ("Variables", 1) => expr.operands[0].variables(),
        ("FreeOf", 2) => expr.operands[0].free_of(&expr.operands[1]),
        ("Substitute", 3) => expr.operands[0].substitute(&expr.operands[1], &expr.operands[2]),
        ("PolynomialExpansion", 4) => polynomial_expansion(
            &expr.operands[0],
            &expr.operands[1],
            &expr.operands[2],
            &expr.operands[3],
        ),
        ("PolynomialGCD", 3) => polynomial_gcd(&expr.operands[0], &expr.operands[1], &expr.operands[2]),
        _ => expr.clone(),
    }
}
