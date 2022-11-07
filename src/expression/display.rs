use std::fmt::{Debug, Display};

use super::{Expr, ExprKind};

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ExprKind::Undefined => write!(f, "undefined"),
            ExprKind::Symbol(ref s) => write!(f, "{}", s),
            ExprKind::Integer(n) => write!(f, "{}", n),
            ExprKind::Fraction(n, d) => write!(f, "{}/{}", n, d),
            ExprKind::Gaussian => {
                if self.operands[0].kind == ExprKind::Integer(0) {
                    if self.operands[1].kind == ExprKind::Integer(1) {
                        write!(f, "\u{1d55a}")
                    } else {
                        write!(f, "{}\u{1d55a}", self.operands[1])
                    }
                } else {
                    if self.operands[1].is_positive_num() {
                        write!(f, "({}+{}\u{1d55a})", self.operands[0], self.operands[1])
                    } else {
                        write!(f, "({}{}\u{1d55a})", self.operands[0], self.operands[1])
                    }
                }
            }
            ExprKind::Sum => {
                let mut s = String::new();
                for (i, operand) in self.operands.iter().enumerate() {
                    if i > 0 {
                        s.push_str(" + ");
                    }
                    s.push_str(&format!("{}", operand));
                }
                write!(f, "({})", s)
            }
            ExprKind::Product => {
                let mut s = String::new();
                for (i, operand) in self.operands.iter().enumerate() {
                    if i > 0 {
                        s.push_str(" * ");
                    }
                    s.push_str(&format!("{}", operand));
                }
                write!(f, "({})", s)
            }
            ExprKind::Difference => {
                write!(f, "({} - {})", self.operands[0], self.operands[1])
            }
            ExprKind::Quotient => {
                write!(f, "({} / {})", self.operands[0], self.operands[1])
            }
            ExprKind::Power => {
                write!(f, "({} ^ {})", self.operands[0], self.operands[1])
            }
            ExprKind::Factorial => {
                write!(f, "({})!", self.operands[0])
            }
            ExprKind::Func(ref name) => {
                let mut s = String::new();
                for (i, operand) in self.operands.iter().enumerate() {
                    if i > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&format!("{}", operand));
                }
                write!(f, "{}({})", name, s)
            }
            ExprKind::Boolean(b) => match b {
                true => write!(f, "True"),
                false => write!(f, "False"),
            },
            ExprKind::Not => write!(f, "!{}", self.operands[0]),
            ExprKind::Or => {
                let mut s = String::new();
                for (i, operand) in self.operands.iter().enumerate() {
                    if i > 0 {
                        s.push_str(" || ");
                    }
                    s.push_str(&format!("{}", operand));
                }
                write!(f, "({})", s)
            }
            ExprKind::And => {
                let mut s = String::new();
                for (i, operand) in self.operands.iter().enumerate() {
                    if i > 0 {
                        s.push_str(" && ");
                    }
                    s.push_str(&format!("{}", operand));
                }
                write!(f, "({})", s)
            }
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
