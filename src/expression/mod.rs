mod display;
mod numeric;
mod polynomial;
mod order;
mod utility;
mod expand;

pub use numeric::*;
pub use polynomial::*;

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

    Boolean(bool),
    Not,
    Or,
    And,

    Set,
    Union,
    Intersection,
    SetDifference,
    Member,
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

    pub fn bool(b: bool) -> Expr {
        Expr {
            kind: ExprKind::Boolean(b),
            operands: vec![],
        }
    }

    pub fn not(expr: Expr) -> Expr {
        Expr {
            kind: ExprKind::Not,
            operands: vec![expr],
        }
    }

    pub fn set(mut expr: Vec<Expr>) -> Expr {
        expr.sort();
        expr.dedup();
        Expr {
            kind: ExprKind::Set,
            operands: expr,
        }
    }
}

