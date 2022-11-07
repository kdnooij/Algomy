use crate::{evaluate::evaluate_quotient, expression::Expr, simplify};

/// Single variable polynomial division. Takes two polynomials and a variable on which they are defined.
/// Returns the quotient and remainder of the division as `(quotient, remainder)`.
pub fn polynomial_division(u: &Expr, v: &Expr, x: &Expr) -> (Expr, Expr) {
    let mut q = Expr::int(0);
    let mut r = u.clone();
    let mut m = r.degree_gpe(x);
    let n = v.degree_gpe(x);
    let lcv = v.leading_coefficient_gpe(x);
    while m >= n {
        let lcr = r.leading_coefficient_gpe(x);
        let s = evaluate_quotient(&lcr, &lcv);
        q = simplify(&Expr::plus(
            q.clone(),
            Expr::times(s.clone(), Expr::power(x.clone(), Expr::int(m - n))),
        ));
        r = simplify(&Expr::minus(
            Expr::minus(
                r.clone(),
                Expr::times(lcr, Expr::power(x.clone(), Expr::int(m))),
            ),
            Expr::product(vec![
                Expr::minus(
                    v.clone(),
                    Expr::times(lcv.clone(), Expr::power(x.clone(), Expr::int(n))),
                ),
                s.clone(),
                Expr::power(x.clone(), Expr::int(m - n)),
            ]),
        ))
        .algebraic_expand();
        m = r.degree_gpe(x);
    }
    return (q, r);
}

pub fn polynomial_quotient(u: &Expr, v: &Expr, x: &Expr) -> Expr {
    let (q, _) = polynomial_division(u, v, x);
    q
}

pub fn polynomial_remainder(u: &Expr, v: &Expr, x: &Expr) -> Expr {
    let (_, r) = polynomial_division(u, v, x);
    r
}
