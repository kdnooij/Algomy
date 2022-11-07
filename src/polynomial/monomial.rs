use crate::{
    expression::{Expr, ExprKind},
    simplify,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Monomial {
    pub coeffs: Vec<Expr>,
    pub vars: Vec<(Expr, i64)>,
}

impl Monomial {
    pub fn new_coeff(coeff: Expr) -> Self {
        Self {
            coeffs: vec![coeff],
            vars: vec![(Expr::int(1), 1)],
        }
    }

    pub fn new_var(var: Expr) -> Self {
        Self {
            coeffs: Vec::new(),
            vars: vec![(var, 1)],
        }
    }

    pub fn new_monomial(coeff: Expr, var: Expr, exp: i64) -> Self {
        Self {
            coeffs: vec![coeff],
            vars: vec![(var, exp)],
        }
    }
}

impl Expr {
    pub fn as_monomial(&self) -> Monomial {
        if self.is_grne() {
            Monomial::new_coeff(self.clone())
        } else {
            match self.kind {
                ExprKind::Symbol(_) => Monomial::new_var(self.clone()),
                ExprKind::Power => {
                    if let ExprKind::Integer(exp) = self.operands[1].kind {
                        Monomial {
                            coeffs: Vec::new(),
                            vars: vec![(self.operands[0].clone(), exp)],
                        }
                    } else {
                        Monomial::new_var(self.clone())
                    }
                }
                ExprKind::Product => {
                    // Split product into coefficients and variables
                    let (coeffs, vars): (Vec<_>, Vec<_>) =
                        self.operands.iter().cloned().partition(|x| x.is_grne());
                    let vars = vars
                        .iter()
                        .map(|v| {
                            if let ExprKind::Power = v.kind {
                                v.as_monomial().vars[0].clone()
                            } else {
                                (v.clone(), 1)
                            }
                        })
                        .collect();
                    Monomial { coeffs, vars }
                }
                ExprKind::Integer(_) | ExprKind::Fraction(_, _) | ExprKind::Gaussian => {
                    unreachable!()
                }
                _ => Monomial::new_var(self.clone()),
            }
        }
    }
}

impl Monomial {
    pub fn degree(&self, var: &Expr) -> i64 {
        self.vars
            .iter()
            .map(|(v, e)| if v == var { *e } else { 0 })
            .sum()
        // TODO: degree(0) = -inf
    }

    pub fn coefficient(&self, var: &Expr, exp: i64) -> Expr {
        if self.vars.len() == 1 && self.vars[0].0 == *var && self.vars[0].1 == exp {
            simplify(&self.coeffs.iter().cloned().product())
        } else {
            Expr::int(0)
        }
    }
}
