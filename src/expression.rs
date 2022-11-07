use std::{
    cmp::Ordering,
    fmt::Debug,
    fmt::Display,
    iter::{Product, Sum},
};

use crate::simplify;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprKind {
    Undefined,
    Func(String),
    Symbol(String),
    Integer(i64),
    Fraction(i64, i64),
    Gaussian,
    Sum,
    Difference,
    Product,
    Quotient,
    Power,
    Factorial,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Expr {
    pub kind: ExprKind,
    pub operands: Vec<Expr>,
}

impl Expr {
    pub fn undefined() -> Expr {
        Expr {
            kind: ExprKind::Undefined,
            operands: vec![],
        }
    }

    pub fn symbol(name: &str) -> Expr {
        Expr {
            kind: ExprKind::Symbol(name.to_string()),
            operands: vec![],
        }
    }

    pub fn int(n: i64) -> Expr {
        Expr {
            kind: ExprKind::Integer(n),
            operands: vec![],
        }
    }

    pub fn frac(n: i64, d: i64) -> Expr {
        Expr {
            kind: ExprKind::Fraction(n, d),
            operands: vec![],
        }
    }

    pub fn power(base: Expr, exponent: Expr) -> Expr {
        Expr {
            kind: ExprKind::Power,
            operands: vec![base, exponent],
        }
    }

    pub fn quotient(n: Expr, d: Expr) -> Expr {
        Expr {
            kind: ExprKind::Quotient,
            operands: vec![n, d],
        }
    }

    pub fn times(lhs: Expr, rhs: Expr) -> Expr {
        Expr {
            kind: ExprKind::Product,
            operands: vec![lhs, rhs],
        }
    }

    pub fn product(operands: Vec<Expr>) -> Expr {
        if operands.len() > 0 {
            Expr {
                kind: ExprKind::Product,
                operands,
            }
        } else {
            Expr::int(1)
        }
    }

    pub fn plus(lhs: Expr, rhs: Expr) -> Expr {
        Expr {
            kind: ExprKind::Sum,
            operands: vec![lhs, rhs],
        }
    }

    pub fn sum(operands: Vec<Expr>) -> Expr {
        if operands.len() > 0 {
            Expr {
                kind: ExprKind::Sum,
                operands,
            }
        } else {
            Expr::int(0)
        }
    }

    pub fn minus(lhs: Expr, rhs: Expr) -> Expr {
        Expr {
            kind: ExprKind::Difference,
            operands: vec![lhs, rhs],
        }
    }

    pub fn factorial(expr: Expr) -> Expr {
        Expr {
            kind: ExprKind::Factorial,
            operands: vec![expr],
        }
    }

    pub fn gaussian(re: Expr, im: Expr) -> Expr {
        Expr {
            kind: ExprKind::Gaussian,
            operands: vec![re, im],
        }
    }

    pub fn function(name: &str, operands: Vec<Expr>) -> Expr {
        Expr {
            kind: ExprKind::Func(name.to_string()),
            operands,
        }
    }
}

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
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

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
