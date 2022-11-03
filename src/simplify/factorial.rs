use crate::expression::{Expr, ExprKind};

pub fn simplify_factorial(expr: &Expr) -> Expr {
    let n = &expr.operands[0];
    match &n.kind {
        ExprKind::Integer(n) if *n >= 0 => Expr::int((1..=*n).product()),
        _ => expr.clone(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{expression::Expr, simplify::factorial::simplify_factorial};

    #[test]
    fn test_simplify_factorial() {
        assert_eq!(
            simplify_factorial(&Expr::factorial(Expr::int(0))),
            Expr::int(1)
        );
        assert_eq!(
            simplify_factorial(&Expr::factorial(Expr::int(5))),
            Expr::int(120)
        );
        assert_eq!(
            simplify_factorial(&Expr::factorial(Expr::int(-5))),
            Expr::factorial(Expr::int(-5))
        );
        assert_eq!(
            simplify_factorial(&Expr::factorial(Expr::frac(3, 2))),
            Expr::factorial(Expr::frac(3, 2))
        );
    }
}
