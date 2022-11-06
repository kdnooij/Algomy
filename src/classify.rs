use crate::expression::{Expr, ExprKind};

impl Expr {
    /// Checks if `self` is an integer.
    pub fn is_integer(&self) -> bool {
        match self.kind {
            ExprKind::Integer(_) => true,
            _ => false,
        }
    }

    /// Checks if `self` is a rational number expression (RNE).
    pub fn is_rne(&self) -> bool {
        if let ExprKind::Integer(_) | ExprKind::Fraction(_, _) = self.kind {
            true
        } else if self.operands.len() == 1 {
            if let ExprKind::Sum | ExprKind::Difference = self.kind {
                self.operands[0].is_rne()
            } else {
                false
            }
        } else if self.operands.len() == 2 {
            match self.kind {
                ExprKind::Sum | ExprKind::Difference | ExprKind::Product | ExprKind::Quotient => {
                    self.operands[0].is_rne() && self.operands[1].is_rne()
                }
                ExprKind::Power => self.operands[0].is_rne() && self.operands[1].is_integer(),
                _ => false,
            }
        } else {
            false
        }
    }

    /// Checks if `self` is a gaussian rational number expression (GRNE).
    pub fn is_grne(&self) -> bool {
        if let ExprKind::Integer(_) | ExprKind::Fraction(_, _) | ExprKind::Gaussian = self.kind {
            true
        } else if self.operands.len() == 1 {
            if let ExprKind::Sum | ExprKind::Difference = self.kind {
                self.operands[0].is_grne()
            } else {
                false
            }
        } else if self.operands.len() == 2 {
            match self.kind {
                ExprKind::Sum | ExprKind::Difference | ExprKind::Product | ExprKind::Quotient => {
                    self.operands[0].is_grne() && self.operands[1].is_grne()
                }
                ExprKind::Power => self.operands[0].is_grne() && self.operands[1].is_integer(),
                _ => false,
            }
        } else {
            false
        }
    }
}
