use crate::{
    expression::{Expr, ExprKind},
    simplify,
};

use self::monomial::Monomial;

mod division;
mod expansion;
mod monomial;
mod gcd;

pub use division::*;
pub use expansion::*;
pub use gcd::*;

impl Expr {
    /// Takes an expression and returns all the monomials in it.
    pub fn as_gpe(&self) -> Vec<Monomial> {
        match self.kind {
            ExprKind::Sum => self.operands.iter().map(|u| u.as_monomial()).collect(),
            _ => vec![self.as_monomial()],
        }
    }

    /// Takes an expression and a variable and returns the degree of that variable
    /// in the expression (when seen as general polynomial expression).
    pub fn degree_gpe(&self, var: &Expr) -> i64 {
        let monomials = self.as_gpe();
        monomials.iter().map(|m| m.degree(var)).max().unwrap_or(0)
    }

    /// Takes an expression, a variable `v` and an exponent `j`. Returns the sum of the coefficients
    /// of all monomials with variable part of the form `v^j` in the expression.
    pub fn coefficient_gpe(&self, var: &Expr, exp: i64) -> Expr {
        let monomials = self.as_gpe();
        simplify(&monomials.iter().map(|m| m.coefficient(var, exp)).sum())
    }

    /// Takes an expression and a variable and returns the leading coefficient of the variable in the expression
    pub fn leading_coefficient_gpe(&self, var: &Expr) -> Expr {
        let degree = self.degree_gpe(var);
        self.coefficient_gpe(var, degree)
    }

    /// Takes an expression and returns all generalized variables in the monomials of the expression.
    /// The result is a set of all generalized variables
    pub fn variables(&self) -> Expr {
        let monomials = self.as_gpe();
        let vars: Vec<Expr> = monomials
            .iter()
            .map(|m| {
                m.vars
                    .iter()
                    .filter(|(v, _)| !v.is_grne())
                    .map(|(v, _)| v.clone())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();
        Expr::set(vars)
    }
}

#[cfg(test)]
mod tests {
    use crate::{expression::Expr, parser::AlgomyKernel, polynomial::Monomial};

    #[test]
    fn test_is_gme() {
        let mut kernel = AlgomyKernel::new();
        let expr = kernel.parse_eval_line("x^2*y^2*z^5");
        assert_eq!(
            expr.as_gpe(),
            vec![Monomial {
                coeffs: Vec::new(),
                vars: vec![
                    (Expr::symbol("x"), 2),
                    (Expr::symbol("y"), 2),
                    (Expr::symbol("z"), 5)
                ],
            }]
        );

        let expr = kernel.parse_eval_line("x^2+y^3*z^2");
        assert_eq!(
            expr.as_gpe(),
            vec![
                Monomial {
                    coeffs: Vec::new(),
                    vars: vec![(Expr::symbol("x"), 2),],
                },
                Monomial {
                    coeffs: Vec::new(),
                    vars: vec![(Expr::symbol("y"), 3), (Expr::symbol("z"), 2)],
                },
            ]
        );

        let expr = kernel.parse_eval_line("(1/3)*x^1+3*y^3+(x+1)");
        assert_eq!(
            expr.as_gpe(),
            vec![
                Monomial::new_coeff(Expr::int(1)),
                Monomial {
                    coeffs: vec![Expr::frac(4, 3)],
                    vars: vec![(Expr::symbol("x"), 1),],
                },
                Monomial {
                    coeffs: vec![Expr::int(3)],
                    vars: vec![(Expr::symbol("y"), 3),],
                },
            ]
        );

        // TODO: Add more tests
    }

    #[test]
    fn test_degree_gpe() {
        let mut kernel = AlgomyKernel::new();
        let expr = kernel.parse_eval_line("(1/3)*x^1+3*y^3+(x+1)");
        assert_eq!(expr.degree_gpe(&Expr::symbol("y")), 3);

        let expr = kernel.parse_eval_line("x^2*y^2*z^5");
        assert_eq!(expr.degree_gpe(&Expr::symbol("z")), 5);

        let expr = kernel.parse_eval_line("x^2*y^2*z^5 + 5");
        assert_eq!(expr.degree_gpe(&Expr::int(1)), 1);

        let expr = kernel.parse_eval_line("x^2*y^2*z^5 + 5");
        assert_eq!(expr.degree_gpe(&Expr::symbol("a")), 0);
    }
}
