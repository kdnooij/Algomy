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

    pub fn free_of(&self, t: &Expr) -> Expr {
        Expr::bool(Expr::free_of_bool(self, t))
    }

    fn free_of_bool(u: &Expr, t: &Expr) -> bool {
        if u == t {
            false
        } else if u.is_atomic() {
            true
        } else {
            u.operands.iter().all(|v| Expr::free_of_bool(v, t))
        }
    }
}

impl Expr {
    pub fn is_undefined(&self) -> bool {
        match self.kind {
            ExprKind::Undefined => true,
            _ => false,
        }
    }

    pub fn is_atomic(&self) -> bool {
        match self.kind {
            ExprKind::Undefined => true,
            ExprKind::Symbol(_) => true,
            ExprKind::Integer(_) => true,
            ExprKind::Fraction(_, _) => true,
            ExprKind::Gaussian => true,
            ExprKind::Boolean(_) => true,
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
