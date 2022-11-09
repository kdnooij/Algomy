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
mod set;
mod sum;

use difference::simplify_difference;
use factorial::simplify_factorial;
use gaussian_number::simplify_gaussian_number;
use power::simplify_power;
use product::simplify_product;
use quotient::simplify_quotient;
use rational_number::simplify_rational_number;
use sum::simplify_sum;

use self::{
    logic::{simplify_and, simplify_not, simplify_or},
    set::{simplify_intersection, simplify_set, simplify_union, simplify_member, simplify_set_difference},
};

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

                ExprKind::Set => simplify_set(&expr),
                ExprKind::Union => simplify_union(&expr),
                ExprKind::Intersection => simplify_intersection(&expr),
                ExprKind::SetDifference => simplify_set_difference(&expr),
                ExprKind::Member => simplify_member(&expr),
                _ => unreachable!(),
            }
        }
    }
}

pub(self) fn merge_nary<F>(p: &[Expr], q: &[Expr], simplify_fn: F) -> Vec<Expr>
where
    F: Fn(&[Expr]) -> Vec<Expr>,
{
    if q.is_empty() {
        p.to_vec()
    } else if p.is_empty() {
        q.to_vec()
    } else {
        let p1 = &p[0];
        let q1 = &q[0];
        let h = simplify_fn(&vec![p1.clone(), q1.clone()]);
        match &h[..] {
            [] => merge_nary(&p[1..], &q[1..], simplify_fn),
            [h1] => {
                let mut r = vec![h1.clone()];
                r.append(&mut merge_nary(&p[1..], &q[1..], simplify_fn));
                r
            }
            [a, b] => {
                if a == p1 && b == q1 {
                    let mut r = vec![p1.clone()];
                    r.append(&mut merge_nary(&p[1..], &q, simplify_fn));
                    r
                } else if a == q1 && b == p1 {
                    let mut r = vec![q1.clone()];
                    r.append(&mut merge_nary(&p, &q[1..], simplify_fn));
                    r
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
}
