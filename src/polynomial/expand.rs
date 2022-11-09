use crate::{
    expression::{Expr, ExprKind},
    simplify::simplify,
};

impl Expr {
    pub fn algebraic_expand(&self) -> Expr {
        match self.kind {
            ExprKind::Sum => match self.operands.len() {
                0 => Expr::int(0),
                1 => self.operands[0].algebraic_expand(),
                _ => simplify(&Expr::plus(
                    self.operands[0].algebraic_expand(),
                    simplify(&Expr::sum(self.operands[1..].to_vec())).algebraic_expand(),
                )),
            },
            ExprKind::Product => match self.operands.len() {
                0 => Expr::int(1),
                1 => self.operands[0].algebraic_expand(),
                _ => expand_product(
                    &self.operands[0].algebraic_expand(),
                    &simplify(&Expr::product(self.operands[1..].to_vec())).algebraic_expand(),
                ),
            },
            ExprKind::Power => {
                let b = self.base();
                let e = self.exponent();
                if let ExprKind::Integer(e) = e.kind {
                    if e >= 2 {
                        expand_power(&b.algebraic_expand(), e)
                    } else {
                        self.clone()
                    }
                } else {
                    self.clone()
                }
            }
            _ => self.clone(),
        }
    }
}

fn expand_product(r: &Expr, s: &Expr) -> Expr {
    match (&r.kind, &s.kind) {
        (ExprKind::Sum, _) => {
            if r.operands.len() >= 2 {
                simplify(&Expr::plus(
                    expand_product(&r.operands[0], s),
                    expand_product(&simplify(&Expr::sum(r.operands[1..].to_vec())), s),
                ))
            } else {
                expand_product(&r.operands[0], s)
            }
        }
        (_, ExprKind::Sum) => expand_product(s, r),
        _ => simplify(&Expr::times(r.clone(), s.clone())),
    }
}

fn expand_power(u: &Expr, n: i64) -> Expr {
    if let ExprKind::Sum = u.kind {
        let f = &u.operands[0];
        let r = simplify(&Expr::sum(u.operands[1..].to_vec()));
        let mut s = Vec::new();
        for k in 0..=n {
            let c = simplify(&Expr::quotient(
                Expr::factorial(Expr::int(n)),
                Expr::times(
                    Expr::factorial(Expr::int(k)),
                    Expr::factorial(Expr::int(n - k)),
                ),
            ));
            s.push(expand_product(
                &simplify(&Expr::times(c, Expr::power(f.clone(), Expr::int(n - k)))),
                &expand_power(&r, k),
            ));
        }
        simplify(&Expr::sum(s))
    } else {
        simplify(&Expr::power(u.clone(), Expr::int(n)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::AlgomyKernel, simplify::simplify};

    #[test]
    pub fn test_algebraic_expand() {
        let mut kernel = AlgomyKernel::new();
        let expr = kernel.parse_eval_line("(x+2)*(x+3)*(x+4)");
        assert_eq!(
            expr.algebraic_expand(),
            kernel.parse_eval_line("x^3+9*x^2+26*x+24")
        );

        let expr = kernel.parse_eval_line("(x+1)^2");
        assert_eq!(expr.algebraic_expand(), kernel.parse_eval_line("1+2*x+x^2"));

        let expr = kernel.parse_eval_line("(x+2)^5*(x+3)^3*(x+4)^2");
        assert_eq!(
            expr.algebraic_expand(),
            kernel.parse_eval_line("13824 + 55296*x + 98784*x^2 + 103760*x^3 + 70944*x^4 + 32984*x^5 + 10558*x^6 + 2297*x^7 + 325*x^8 + 27*x^9 + x^10")
        );

        let expr = kernel.parse_eval_line("(x*(y+1)^3+1)*(x*(y+1)^2+1)");
        assert_eq!(
            expr.algebraic_expand(),
            kernel.parse_eval_line("1 + 2*x + x^2 + 5*x*y + 5*x^2*y + 4*x*y^2 + 10*x^2*y^2 + x*y^3 + 10*x^2*y^3 + 5*x^2*y^4 + x^2*y^5")
        );
    }
}
