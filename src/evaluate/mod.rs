use crate::expression::Expr;

mod gaussian;
mod function;

pub use gaussian::*;
pub use function::*;

/// Evaluates v + w where `v` and `w` are integers or fractions
pub fn evaluate_sum(v: &Expr, w: &Expr) -> Expr {
    Expr::frac(
        v.numerator_rne() * w.denominator_rne() + w.numerator_rne() * v.denominator_rne(),
        v.denominator_rne() * w.denominator_rne(),
    )
}

/// Evaluates v - w where `v` and `w` are integers or fractions
pub fn evaluate_difference(v: &Expr, w: &Expr) -> Expr {
    Expr::frac(
        v.numerator_rne() * w.denominator_rne() - w.numerator_rne() * v.denominator_rne(),
        v.denominator_rne() * w.denominator_rne(),
    )
}

/// Evaluates v * w where `v` and `w` are integers or fractions
pub fn evaluate_product(v: &Expr, w: &Expr) -> Expr {
    Expr::frac(v.numerator_rne() * w.numerator_rne(), v.denominator_rne() * w.denominator_rne())
}

/// Evaluates v/w where `v` and `w` are integers or fractions
pub fn evaluate_quotient(v: &Expr, w: &Expr) -> Expr {
    if w.numerator_rne() == 0 {
        Expr::undefined()
    } else {
        Expr::frac(
            v.numerator_rne() * w.denominator_rne(),
            v.denominator_rne() * w.numerator_rne(),
        )
    }
}

/// Evaluates v^n where `v` is an integer or fraction with non-zero denominator and `n` is an integer
pub fn evaluate_power(v: &Expr, n: i64) -> Expr {
    if v.numerator_rne() != 0 {
        if n > 0 {
            let s = evaluate_power(v, n - 1);
            evaluate_product(&s, v)
        } else if n == 0 {
            Expr::int(1)
        } else if n == -1 {
            Expr::frac(v.denominator_rne(), v.numerator_rne())
        } else {
            // n < -1
            let s = Expr::frac(v.denominator_rne(), v.numerator_rne());
            evaluate_power(&s, -n)
        }
    } else {
        // v.numerator() == 0
        if n >= 1 {
            Expr::int(0)
        } else {
            // n <= 0
            Expr::undefined()
        }
    }
}
