use crate::expression::{Expr, ExprKind};

mod gaussian_number;
mod difference;
mod factorial;
mod power;
mod product;
mod quotient;
mod rational_number;
mod sum;

use power::simplify_power;
use product::simplify_product;
use rational_number::simplify_rational_number;
use sum::simplify_sum;

use self::{
    gaussian_number::simplify_gaussian_number, difference::simplify_difference,
    factorial::simplify_factorial, quotient::simplify_quotient,
};

pub fn simplify(expr: &Expr) -> Expr {
    match &expr.kind {
        ExprKind::Integer(_) | ExprKind::Symbol(_) => expr.clone(),
        ExprKind::Fraction(_, _) => simplify_rational_number(expr),
        ExprKind::Gaussian => simplify_gaussian_number(expr),
        kind => {
            let expr = expr.map(simplify);
            match kind {
                ExprKind::Power => simplify_power(&expr),
                ExprKind::Product => simplify_product(&expr),
                ExprKind::Sum => simplify_sum(&expr),
                ExprKind::Quotient => simplify_quotient(&expr),
                ExprKind::Difference => simplify_difference(&expr),
                ExprKind::Factorial => simplify_factorial(&expr),
                ExprKind::Func(_) => expr,
                _ => unreachable!(),
            }
        }
    }
}
