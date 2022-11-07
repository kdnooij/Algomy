use crate::simplify;

use super::{Expr, ExprKind};

impl Expr {
    pub fn is_positive_num(&self) -> bool {
        match self.kind {
            ExprKind::Integer(n) => n > 0,
            ExprKind::Fraction(n, d) => !((n > 0) ^ (d > 0)),
            _ => unreachable!(),
        }
    }

    pub fn numerator_rne(&self) -> i64 {
        match self.kind {
            ExprKind::Integer(n) => n,
            ExprKind::Fraction(n, _) => n,
            _ => unreachable!(),
        }
    }

    pub fn denominator_rne(&self) -> i64 {
        match self.kind {
            ExprKind::Integer(_) => 1,
            ExprKind::Fraction(_, d) => d,
            _ => unreachable!(),
        }
    }

    pub fn numerator(&self) -> Expr {
        match self.kind {
            ExprKind::Integer(_) | ExprKind::Fraction(_, _) => Expr::int(self.numerator_rne()),
            ExprKind::Quotient => self.operands[0].clone(),
            ExprKind::Product => simplify(&self.operands.iter().map(|x| x.numerator()).product()),
            _ => self.clone(),
        }
    }

    pub fn denominator(&self) -> Expr {
        match self.kind {
            ExprKind::Integer(_) | ExprKind::Fraction(_, _) => Expr::int(self.denominator_rne()),
            ExprKind::Quotient => self.operands[1].clone(),
            ExprKind::Product => simplify(&self.operands.iter().map(|x| x.denominator()).product()),
            _ => self.clone(),
        }
    }

    pub fn base(&self) -> Expr {
        match self.kind {
            ExprKind::Power => self.operands[0].clone(),
            _ => self.clone(),
        }
    }

    pub fn exponent(&self) -> Expr {
        match self.kind {
            ExprKind::Power => self.operands[1].clone(),
            _ => Expr::int(1),
        }
    }

    /// Returns the real part of a gaussian number
    pub fn re(&self) -> Expr {
        match self.kind {
            ExprKind::Integer(_) | ExprKind::Fraction(_, _) => self.clone(),
            ExprKind::Gaussian => self.operands[0].clone(),
            ExprKind::Sum => simplify(&self.operands.iter().map(|u| u.re()).sum()),
            ExprKind::Product => simplify(&self.operands.iter().map(|u| u.re()).product()),
            _ => Expr::function("Re", vec![self.clone()]),
        }
    }

    /// Returns the imaginary part of a gaussian number
    pub fn im(&self) -> Expr {
        match self.kind {
            ExprKind::Integer(_) | ExprKind::Fraction(_, _) => Expr::int(0),
            ExprKind::Gaussian => self.operands[1].clone(),
            ExprKind::Sum => simplify(&self.operands.iter().map(|u| u.im()).sum()),
            ExprKind::Product => simplify(&self.operands.iter().map(|u| u.im()).product()),
            _ => Expr::function("Im", vec![self.clone()]),
        }
    }
}
