use crate::expression::{Expr, ExprKind};

mod power;
mod product;
mod rational_number;
mod sum;
mod quotient;
mod difference;
mod factorial;

use power::simplify_power;
use product::simplify_product;
use rational_number::simplify_rational_number;
use sum::simplify_sum;

use self::{quotient::simplify_quotient, difference::simplify_difference, factorial::simplify_factorial};

pub fn simplify(expr: &Expr) -> Expr {
    match &expr.kind {
        ExprKind::Integer(_) | ExprKind::Symbol(_) => expr.clone(),
        ExprKind::Fraction(_, _) => simplify_rational_number(expr),
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
