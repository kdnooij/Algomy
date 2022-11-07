use super::{Expr, ExprKind};

impl Expr {
    /// Returns the leading coefficient of a product expression
    pub fn product_coeff(&self) -> Expr {
        match self.kind {
            ExprKind::Product => match self.operands[0].kind {
                // TODO: We assume the coefficient is the leading term. This is probably not always true.
                ExprKind::Integer(_) | ExprKind::Fraction(_, _) => self.operands[0].clone(),
                _ => Expr::int(1),
            },
            _ => Expr::int(1),
        }
    }

    /// Returns the variable part of a product expression
    pub fn product_rest(&self) -> Expr {
        match self.kind {
            ExprKind::Product => match self.operands[0].kind {
                // TODO: We assume the coefficient is the leading term. This is probably not always true.
                ExprKind::Integer(_) | ExprKind::Fraction(_, _) => {
                    if self.operands.len() == 2 {
                        self.operands[1].clone()
                    } else {
                        Expr {
                            kind: ExprKind::Product,
                            operands: self.operands[1..].to_vec(),
                        }
                    }
                }
                _ => self.clone(),
            },
            _ => self.clone(),
        }
    }
}