use crate::expression::Expr;

mod gaussian;
pub use gaussian::*;

/// Evaluates v + w where `v` and `w` are integers or fractions
pub fn evaluate_sum(v: &Expr, w: &Expr) -> Expr {
    Expr::frac(
        v.numerator() * w.denominator() + w.numerator() * v.denominator(),
        v.denominator() * w.denominator(),
    )
}

/// Evaluates v - w where `v` and `w` are integers or fractions
pub fn evaluate_difference(v: &Expr, w: &Expr) -> Expr {
    Expr::frac(
        v.numerator() * w.denominator() - w.numerator() * v.denominator(),
        v.denominator() * w.denominator(),
    )
}

/// Evaluates v * w where `v` and `w` are integers or fractions
pub fn evaluate_product(v: &Expr, w: &Expr) -> Expr {
    Expr::frac(v.numerator() * w.numerator(), v.denominator() * w.denominator())
}

/// Evaluates v/w where `v` and `w` are integers or fractions
pub fn evaluate_quotient(v: &Expr, w: &Expr) -> Expr {
    if w.numerator() == 0 {
        Expr::undefined()
    } else {
        Expr::frac(
            v.numerator() * w.denominator(),
            v.denominator() * w.numerator(),
        )
    }
}

/// Evaluates v^n where `v` is an integer or fraction with non-zero denominator and `n` is an integer
pub fn evaluate_power(v: &Expr, n: i64) -> Expr {
    if v.numerator() != 0 {
        if n > 0 {
            let s = evaluate_power(v, n - 1);
            evaluate_product(&s, v)
        } else if n == 0 {
            Expr::int(1)
        } else if n == -1 {
            Expr::frac(v.denominator(), v.numerator())
        } else {
            // n < -1
            let s = Expr::frac(v.denominator(), v.numerator());
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
