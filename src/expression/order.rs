use std::cmp::Ordering;

use super::{Expr, ExprKind};

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.kind, &other.kind) {
            // 1 < 4, 1/2 < 2
            (
                ExprKind::Integer(_) | ExprKind::Fraction(_, _),
                ExprKind::Integer(_) | ExprKind::Fraction(_, _),
            ) => {
                let n1 = self.numerator_rne();
                let d1 = self.denominator_rne();
                let n2 = other.numerator_rne();
                let d2 = other.denominator_rne();
                (n1 * d2).cmp(&(n2 * d1))
            }
            (ExprKind::Gaussian, ExprKind::Gaussian) => match self.re().cmp(&other.re()) {
                Ordering::Equal => self.im().cmp(&other.im()),
                ord => ord,
            },
            (ExprKind::Boolean(b1), ExprKind::Boolean(b2)) => b1.cmp(b2),
            // 0..9 < A..Z < a..z
            (ExprKind::Symbol(ref s1), ExprKind::Symbol(ref s2)) => s1.cmp(s2),
            (ExprKind::Sum, ExprKind::Sum) | (ExprKind::Product, ExprKind::Product) => {
                let m = self.operands.len() - 1;
                let n = other.operands.len() - 1;
                if self.operands[m] != other.operands[n] {
                    self.operands[m].cmp(&other.operands[n])
                } else {
                    for k in 1..m.min(n) {
                        if self.operands[m - k] != other.operands[n - k] {
                            return self.operands[m - k].cmp(&other.operands[n - k]);
                        }
                    }
                    m.cmp(&n)
                }
            }
            (ExprKind::Power, ExprKind::Power) => {
                if self.base() != other.base() {
                    self.base().cmp(&other.base())
                } else {
                    self.exponent().cmp(&other.exponent())
                }
            }
            (ExprKind::And, ExprKind::And) | (ExprKind::Or, ExprKind::Or) => {
                let m = self.operands.len() - 1;
                let n = other.operands.len() - 1;
                if self.operands[m] != other.operands[n] {
                    self.operands[m].cmp(&other.operands[n])
                } else {
                    for k in 1..m.min(n) {
                        if self.operands[m - k] != other.operands[n - k] {
                            return self.operands[m - k].cmp(&other.operands[n - k]);
                        }
                    }
                    m.cmp(&n)
                }
            }
            (ExprKind::Factorial, ExprKind::Factorial) => self.operands[0].cmp(&other.operands[0]),
            (ExprKind::Func(ref f1), ExprKind::Func(ref f2)) => {
                let m = self.operands.len() - 1;
                let n = other.operands.len() - 1;
                if f1 != f2 {
                    f1.cmp(f2)
                } else if self.operands[0] != other.operands[0] {
                    self.operands[0].cmp(&other.operands[0])
                } else {
                    for k in 1..=m.min(n) {
                        if self.operands[k] != other.operands[k] {
                            return self.operands[k].cmp(&other.operands[k]);
                        }
                    }
                    m.cmp(&n)
                }
            }
            (ExprKind::Integer(_) | ExprKind::Fraction(_, _), _) => Ordering::Less,
            (ExprKind::Gaussian, _) => Ordering::Less,
            (ExprKind::Boolean(_), _) => Ordering::Less,
            (
                ExprKind::And,
                ExprKind::Or
                | ExprKind::Not
                | ExprKind::Product
                | ExprKind::Sum
                | ExprKind::Factorial
                | ExprKind::Func(_)
                | ExprKind::Symbol(_),
            ) => self.cmp(&Expr {
                kind: ExprKind::And,
                operands: vec![other.clone()],
            }),
            (
                ExprKind::Or,
                ExprKind::Not
                | ExprKind::Product
                | ExprKind::Sum
                | ExprKind::Factorial
                | ExprKind::Func(_)
                | ExprKind::Symbol(_),
            ) => self.cmp(&Expr {
                kind: ExprKind::Or,
                operands: vec![other.clone()],
            }),
            (
                ExprKind::Not,
                ExprKind::Product
                | ExprKind::Sum
                | ExprKind::Factorial
                | ExprKind::Func(_)
                | ExprKind::Symbol(_),
            ) => {
                if self.operands[0] == *other {
                    Ordering::Greater
                } else {
                    self.cmp(&Expr::not(other.clone()))
                }
            }
            (
                ExprKind::Product,
                ExprKind::Power
                | ExprKind::Sum
                | ExprKind::Factorial
                | ExprKind::Func(_)
                | ExprKind::Symbol(_),
            ) => self.cmp(&Expr {
                kind: ExprKind::Product,
                operands: vec![other.clone()],
            }),
            (
                ExprKind::Power,
                ExprKind::Sum | ExprKind::Factorial | ExprKind::Func(_) | ExprKind::Symbol(_),
            ) => self.cmp(&Expr::power(other.clone(), Expr::int(1))),
            (ExprKind::Sum, ExprKind::Factorial | ExprKind::Func(_) | ExprKind::Symbol(_)) => self
                .cmp(&Expr {
                    kind: ExprKind::Sum,
                    operands: vec![other.clone()],
                }),
            (ExprKind::Factorial, ExprKind::Func(_) | ExprKind::Symbol(_)) => {
                if self.operands[0] == *other {
                    Ordering::Greater
                } else {
                    self.cmp(&Expr::factorial(other.clone()))
                }
            }
            (ExprKind::Func(ref f), ExprKind::Symbol(ref s)) => {
                if f == s {
                    Ordering::Greater
                } else {
                    f.cmp(&s)
                }
            }
            _ => match other.cmp(self) {
                Ordering::Less => Ordering::Greater,
                Ordering::Greater => Ordering::Less,
                Ordering::Equal => Ordering::Equal,
            },
        }
    }
}