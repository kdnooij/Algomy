use crate::expression::{Expr, ExprKind};

use super::merge_nary;

pub fn simplify_not(expr: &Expr) -> Expr {
    let u = &expr.operands[0];

    match &u.kind {
        ExprKind::Boolean(b) => Expr::bool(!b),
        ExprKind::Not => u.operands[0].clone(),
        ExprKind::Undefined => Expr::undefined(),
        _ => expr.clone(),
    }
}

pub fn simplify_or(u: &Expr) -> Expr {
    if u.operands.iter().find(|v| v.is_undefined()).is_some() {
        // ... || undefined -> undefined
        Expr::undefined()
    } else if u
        .operands
        .iter()
        .find(|v| v.kind == ExprKind::Boolean(true))
        .is_some()
    {
        // ... || True -> True
        Expr::bool(true)
    } else if u.operands.len() == 1 {
        // ||a -> a
        u.operands[0].clone()
    } else {
        let v: Vec<Expr> = simplify_or_recursive(&u.operands);
        match v.len() {
            0 => Expr::bool(false),
            1 => v[0].clone(),
            _ => Expr {
                kind: ExprKind::Or,
                operands: v,
            },
        }
    }
}

fn simplify_or_recursive(l: &[Expr]) -> Vec<Expr> {
    if l.len() == 2 && l[0].kind != ExprKind::Or && l[1].kind != ExprKind::Or {
        let u1 = &l[0];
        let u2 = &l[1];
        match (&u1.kind, &u2.kind) {
            (ExprKind::Boolean(b1), ExprKind::Boolean(b2)) => {
                vec![Expr::bool(*b1 || *b2)]
            }
            (ExprKind::Boolean(false), _) => {
                vec![u2.clone()]
            }
            (_, ExprKind::Boolean(false)) => {
                vec![u1.clone()]
            }
            _ => {
                if u2 < u1 {
                    vec![u2.clone(), u1.clone()]
                } else {
                    l.to_vec()
                }
            }
        }
    } else if l.len() == 2 && (l[0].kind == ExprKind::Or || l[1].kind == ExprKind::Or) {
        let u1 = &l[0];
        let u2 = &l[1];
        match (&u1.kind, &u2.kind) {
            (ExprKind::Or, ExprKind::Or) => {
                merge_nary(&u1.operands, &u2.operands, simplify_or_recursive)
            }
            (ExprKind::Or, _) => merge_nary(&u1.operands, &[u2.clone()], simplify_or_recursive),
            (_, ExprKind::Or) => merge_nary(&[u1.clone()], &u2.operands, simplify_or_recursive),
            _ => unreachable!(),
        }
    } else {
        // l.len() > 2
        let w = simplify_or_recursive(&l[1..]);
        if let ExprKind::Or = l[0].kind {
            merge_nary(&l[0].operands, &w, simplify_or_recursive)
        } else {
            merge_nary(&[l[0].clone()], &w, simplify_or_recursive)
        }
    }
}

pub fn simplify_and(u: &Expr) -> Expr {
    if u.operands.iter().find(|v| v.is_undefined()).is_some() {
        // ... && undefined -> undefined
        Expr::undefined()
    } else if u
        .operands
        .iter()
        .find(|v| v.kind == ExprKind::Boolean(false))
        .is_some()
    {
        // ... && False -> False
        Expr::bool(false)
    } else if u.operands.len() == 1 {
        // &&a -> a
        u.operands[0].clone()
    } else {
        let v: Vec<Expr> = simplify_and_recursive(&u.operands);
        match v.len() {
            0 => Expr::bool(true),
            1 => v[0].clone(),
            _ => Expr {
                kind: ExprKind::And,
                operands: v,
            },
        }
    }
}

fn simplify_and_recursive(l: &[Expr]) -> Vec<Expr> {
    if l.len() == 2 && l[0].kind != ExprKind::And && l[1].kind != ExprKind::And {
        let u1 = &l[0];
        let u2 = &l[1];
        match (&u1.kind, &u2.kind) {
            (ExprKind::Boolean(b1), ExprKind::Boolean(b2)) => {
                vec![Expr::bool(*b1 && *b2)]
            }
            (ExprKind::Boolean(true), _) => {
                vec![u2.clone()]
            }
            (_, ExprKind::Boolean(true)) => {
                vec![u1.clone()]
            }
            _ => {
                if u2 < u1 {
                    vec![u2.clone(), u1.clone()]
                } else {
                    l.to_vec()
                }
            }
        }
    } else if l.len() == 2 && (l[0].kind == ExprKind::And || l[1].kind == ExprKind::And) {
        let u1 = &l[0];
        let u2 = &l[1];
        match (&u1.kind, &u2.kind) {
            (ExprKind::And, ExprKind::And) => {
                merge_nary(&u1.operands, &u2.operands, simplify_and_recursive)
            }
            (ExprKind::And, _) => merge_nary(&u1.operands, &[u2.clone()], simplify_and_recursive),
            (_, ExprKind::And) => merge_nary(&[u1.clone()], &u2.operands, simplify_and_recursive),
            _ => unreachable!(),
        }
    } else {
        // l.len() > 2
        let w = simplify_and_recursive(&l[1..]);
        if let ExprKind::And = l[0].kind {
            merge_nary(&l[0].operands, &w, simplify_and_recursive)
        } else {
            merge_nary(&[l[0].clone()], &w, simplify_and_recursive)
        }
    }
}
