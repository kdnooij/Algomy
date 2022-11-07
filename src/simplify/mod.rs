use crate::{
    evaluate::evaluate_function,
    expression::{Expr, ExprKind},
};

mod difference;
mod factorial;
mod gaussian_number;
mod logic;
mod power;
mod product;
mod quotient;
mod rational_number;
mod sum;

use difference::simplify_difference;
use factorial::simplify_factorial;
use gaussian_number::simplify_gaussian_number;
use power::simplify_power;
use product::simplify_product;
use quotient::simplify_quotient;
use rational_number::simplify_rational_number;
use sum::simplify_sum;

use self::logic::{simplify_not, simplify_or, simplify_and};

pub fn simplify(expr: &Expr) -> Expr {
    match &expr.kind {
        ExprKind::Integer(_) | ExprKind::Symbol(_) | ExprKind::Boolean(_) => expr.clone(),
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
                ExprKind::Func(_) => evaluate_function(&expr),

                ExprKind::Not => simplify_not(&expr),
                ExprKind::Or => simplify_or(&expr),
                ExprKind::And => simplify_and(&expr),
                _ => unreachable!(),
            }
        }
    }
}
