use crate::expression::{Expr, ExprKind};

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
            (ExprKind::Or, ExprKind::Or) => merge_ors(&u1.operands, &u2.operands),
            (ExprKind::Or, _) => merge_ors(&u1.operands, &[u2.clone()]),
            (_, ExprKind::Or) => merge_ors(&[u1.clone()], &u2.operands),
            _ => unreachable!(),
        }
    } else {
        // l.len() > 2
        let w = simplify_or_recursive(&l[1..]);
        if let ExprKind::Or = l[0].kind {
            merge_ors(&l[0].operands, &w)
        } else {
            merge_ors(&[l[0].clone()], &w)
        }
    }
}

fn merge_ors(p: &[Expr], q: &[Expr]) -> Vec<Expr> {
    if q.is_empty() {
        p.to_vec()
    } else if p.is_empty() {
        q.to_vec()
    } else {
        let p1 = &p[0];
        let q1 = &q[0];
        let h = simplify_or_recursive(&vec![p1.clone(), q1.clone()]);
        match &h[..] {
            [] => merge_ors(&p[1..], &q[1..]),
            [h1] => {
                let mut r = vec![h1.clone()];
                r.append(&mut merge_ors(&p[1..], &q[1..]));
                r
            }
            [a, b] => {
                if a == p1 && b == q1 {
                    let mut r = vec![p1.clone()];
                    r.append(&mut merge_ors(&p[1..], &q));
                    r
                } else if a == q1 && b == p1 {
                    let mut r = vec![q1.clone()];
                    r.append(&mut merge_ors(&p, &q[1..]));
                    r
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
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
            (ExprKind::And, ExprKind::And) => merge_ands(&u1.operands, &u2.operands),
            (ExprKind::And, _) => merge_ands(&u1.operands, &[u2.clone()]),
            (_, ExprKind::And) => merge_ands(&[u1.clone()], &u2.operands),
            _ => unreachable!(),
        }
    } else {
        // l.len() > 2
        let w = simplify_and_recursive(&l[1..]);
        if let ExprKind::And = l[0].kind {
            merge_ands(&l[0].operands, &w)
        } else {
            merge_ands(&[l[0].clone()], &w)
        }
    }
}

fn merge_ands(p: &[Expr], q: &[Expr]) -> Vec<Expr> {
    if q.is_empty() {
        p.to_vec()
    } else if p.is_empty() {
        q.to_vec()
    } else {
        let p1 = &p[0];
        let q1 = &q[0];
        let h = simplify_and_recursive(&vec![p1.clone(), q1.clone()]);
        match &h[..] {
            [] => merge_ands(&p[1..], &q[1..]),
            [h1] => {
                let mut r = vec![h1.clone()];
                r.append(&mut merge_ands(&p[1..], &q[1..]));
                r
            }
            [a, b] => {
                if a == p1 && b == q1 {
                    let mut r = vec![p1.clone()];
                    r.append(&mut merge_ands(&p[1..], &q));
                    r
                } else if a == q1 && b == p1 {
                    let mut r = vec![q1.clone()];
                    r.append(&mut merge_ands(&p, &q[1..]));
                    r
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
}