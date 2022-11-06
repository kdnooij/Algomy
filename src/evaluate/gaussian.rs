use crate::expression::Expr;

use super::{
    evaluate_difference, evaluate_power, evaluate_product, evaluate_quotient, evaluate_sum,
};

/// Evaluates v + w where `v` and `w` are integers, fractions or gaussian numbers
pub fn evaluate_sum_gaussian(v: &Expr, w: &Expr) -> Expr {
    let v_re = v.re();
    let v_im = v.im();
    let w_re = w.re();
    let w_im = w.im();
    Expr::gaussian(
        evaluate_sum(&v_re, &w_re),
        evaluate_sum(&v_im, &w_im),
    )
}

/// Evaluates v - w where `v` and `w` are integers, fractions or gaussian numbers
pub fn evaluate_difference_gaussian(v: &Expr, w: &Expr) -> Expr {
    let v_re = v.re();
    let v_im = v.im();
    let w_re = w.re();
    let w_im = w.im();
    Expr::gaussian(
        evaluate_difference(&v_re, &w_re),
        evaluate_sum(&v_im, &w_im),
    )
}

/// Evaluates v * w where `v` and `w` are integers, fractions or gaussian numbers
pub fn evaluate_product_gaussian(v: &Expr, w: &Expr) -> Expr {
    let v_re = v.re();
    let v_im = v.im();
    let w_re = w.re();
    let w_im = w.im();
    Expr::gaussian(
        evaluate_difference(&evaluate_product(&v_re, &w_re), &evaluate_product(&v_im, &w_im)),
        evaluate_sum(&evaluate_product(&v_re, &w_im), &evaluate_product(&v_im, &w_re)),
    )
}

/// Evaluates v/w where `v` and `w` are integers, fractions or gaussian numbers
pub fn evaluate_quotient_gaussian(v: &Expr, w: &Expr) -> Expr {
    if w.re().numerator() == 0 && w.im().numerator() == 0 {
        Expr::undefined()
    } else {
        let v_re = v.re(); // a
        let v_im = v.im(); // b
        let w_re = w.re(); // c
        let w_im = w.im(); // d

        let d = evaluate_sum(&evaluate_power(&w_re, 2), &evaluate_power(&w_im, 2));
        Expr::gaussian(
            evaluate_quotient(
                &evaluate_sum(
                    &evaluate_product(&v_re, &w_re),
                    &evaluate_product(&v_im, &w_im),
                ),
                &d,
            ),
            evaluate_quotient(
                &evaluate_difference(
                    &evaluate_product(&v_im, &w_re),
                    &evaluate_product(&v_re, &w_im),
                ),
                &d,
            ),
        )
    }
}

/// Evaluates v^n where `v` is an integer, fraction or gaussian number with non-zero denominator and `n` is an integer
pub fn evaluate_power_gaussian(v: &Expr, n: i64) -> Expr {
    if v.re().numerator() != 0 || v.im().numerator() != 0 {
        if n > 0 {
            let s = evaluate_power_gaussian(v, n - 1);
            evaluate_product_gaussian(&s, v)
        } else if n == 0 {
            Expr::int(1)
        } else if n == -1 {
            evaluate_quotient_gaussian(&Expr::int(1), v)
        } else {
            // n < -1
            let s = evaluate_quotient_gaussian(&Expr::int(1), v);
            evaluate_power_gaussian(&s, -n)
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