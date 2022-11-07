use std::iter::{Product, Sum};

use super::{Expr, ExprKind};

impl Expr {
    pub fn map<F>(&self, f: F) -> Expr
    where
        F: Fn(&Expr) -> Expr,
    {
        let mut expr = self.clone();
        expr.operands = expr.operands.iter().map(f).collect();
        expr
    }
}

impl Expr {
    pub fn is_undefined(&self) -> bool {
        match self.kind {
            ExprKind::Undefined => true,
            _ => false,
        }
    }

    pub fn num_operands(&self) -> usize {
        self.operands.len()
    }
}

impl Sum for Expr {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let operands: Vec<_> = iter.collect();
        match operands.len() {
            0 => Expr::int(0),
            1 => operands[0].clone(),
            _ => Expr {
                kind: ExprKind::Sum,
                operands,
            },
        }
    }
}

impl Product for Expr {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        let operands: Vec<_> = iter.collect();
        match operands.len() {
            0 => Expr::int(1),
            1 => operands[0].clone(),
            _ => Expr {
                kind: ExprKind::Product,
                operands,
            },
        }
    }
}
