use crate::expression::{Expr, ExprKind};

use super::merge_nary;

pub fn simplify_set(expr: &Expr) -> Expr {
    let mut v: Vec<Expr> = expr.operands.clone();
    v.sort();
    v.dedup();
    Expr {
        kind: ExprKind::Set,
        operands: v,
    }
}

pub fn simplify_union(u: &Expr) -> Expr {
    if u.operands.len() == 1 {
        u.operands[0].clone()
    } else {
        let v: Vec<Expr> = simplify_union_recursive(&u.operands);
        match v.len() {
            1 => v[0].clone(),
            _ => Expr {
                kind: ExprKind::Union,
                operands: v,
            },
        }
    }
}

fn simplify_union_recursive(l: &[Expr]) -> Vec<Expr> {
    if l.len() == 2 && l[0].kind != ExprKind::Union && l[1].kind != ExprKind::Union {
        let u1 = &l[0];
        let u2 = &l[1];
        match (&u1.kind, &u2.kind) {
            (ExprKind::Set, ExprKind::Set) => {
                vec![simplify_set(&Expr {
                    kind: ExprKind::Set,
                    operands: [&u1.operands[..], &u2.operands[..]].concat(),
                })]
            }
            _ => {
                if u2 < u1 {
                    vec![u2.clone(), u1.clone()]
                } else {
                    l.to_vec()
                }
            }
        }
    } else if l.len() == 2 && (l[0].kind == ExprKind::Union || l[1].kind == ExprKind::Union) {
        let u1 = &l[0];
        let u2 = &l[1];
        match (&u1.kind, &u2.kind) {
            (ExprKind::Union, ExprKind::Union) => {
                merge_nary(&u1.operands, &u2.operands, simplify_union_recursive)
            }
            (ExprKind::Union, _) => {
                merge_nary(&u1.operands, &[u2.clone()], simplify_union_recursive)
            }
            (_, ExprKind::Union) => {
                merge_nary(&[u1.clone()], &u2.operands, simplify_union_recursive)
            }
            _ => unreachable!(),
        }
    } else {
        // l.len() > 2
        let w = simplify_union_recursive(&l[1..]);
        if let ExprKind::Union = l[0].kind {
            merge_nary(&l[0].operands, &w, simplify_union_recursive)
        } else {
            merge_nary(&[l[0].clone()], &w, simplify_union_recursive)
        }
    }
}

pub fn simplify_intersection(u: &Expr) -> Expr {
    let has_empty_set = u
        .operands
        .iter()
        .find(|v| {
            if let ExprKind::Set = v.kind {
                v.operands.len() == 0
            } else {
                false
            }
        })
        .is_some();
    if has_empty_set {
        Expr {
            kind: ExprKind::Set,
            operands: vec![],
        }
    } else if u.operands.len() == 1 {
        u.operands[0].clone()
    } else {
        let v: Vec<Expr> = simplify_intersection_recursive(&u.operands);
        match v.len() {
            1 => v[0].clone(),
            _ => Expr {
                kind: ExprKind::Intersection,
                operands: v,
            },
        }
    }
}

fn intersect_sorted(l: &[Expr], r: &[Expr]) -> Vec<Expr> {
    let mut i = 0;
    let mut j = 0;
    let mut v = Vec::new();
    while i < l.len() && j < r.len() {
        if l[i] == r[j] {
            v.push(l[i].clone());
            i += 1;
            j += 1;
        } else if l[i] < r[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    v
}

fn simplify_intersection_recursive(l: &[Expr]) -> Vec<Expr> {
    if l.len() == 2 && l[0].kind != ExprKind::Intersection && l[1].kind != ExprKind::Intersection {
        let u1 = &l[0];
        let u2 = &l[1];
        match (&u1.kind, &u2.kind) {
            (ExprKind::Set, ExprKind::Set) => {
                vec![Expr {
                    kind: ExprKind::Set,
                    operands: intersect_sorted(&u1.operands[..], &u2.operands[..]),
                }]
            }
            _ => {
                if u2 < u1 {
                    vec![u2.clone(), u1.clone()]
                } else {
                    l.to_vec()
                }
            }
        }
    } else if l.len() == 2
        && (l[0].kind == ExprKind::Intersection || l[1].kind == ExprKind::Intersection)
    {
        let u1 = &l[0];
        let u2 = &l[1];
        match (&u1.kind, &u2.kind) {
            (ExprKind::Intersection, ExprKind::Intersection) => {
                merge_nary(&u1.operands, &u2.operands, simplify_intersection_recursive)
            }
            (ExprKind::Intersection, _) => {
                merge_nary(&u1.operands, &[u2.clone()], simplify_intersection_recursive)
            }
            (_, ExprKind::Intersection) => {
                merge_nary(&[u1.clone()], &u2.operands, simplify_intersection_recursive)
            }
            _ => unreachable!(),
        }
    } else {
        // l.len() > 2
        let w = simplify_intersection_recursive(&l[1..]);
        if let ExprKind::Intersection = l[0].kind {
            merge_nary(&l[0].operands, &w, simplify_intersection_recursive)
        } else {
            merge_nary(&[l[0].clone()], &w, simplify_intersection_recursive)
        }
    }
}

pub fn simplify_set_difference(expr: &Expr) -> Expr {
    let u = &expr.operands[0];
    let v = &expr.operands[1];
    match (&u.kind, &v.kind) {
        (ExprKind::Set, ExprKind::Set) => {
            if u.operands.len() == 0 || v.operands.len() == 0 {
                u.clone()
            } else {
                Expr {
                    kind: ExprKind::Set,
                    operands: set_difference_sorted(&u.operands[..], &v.operands[..]),
                }
            }
        }
        _ => Expr {
            kind: ExprKind::SetDifference,
            operands: vec![u.clone(), v.clone()],
        },
    }
}

fn set_difference_sorted(l: &[Expr], r: &[Expr]) -> Vec<Expr> {
    let mut i = 0;
    let mut j = 0;
    let mut v = Vec::new();
    while i < l.len() && j < r.len() {
        if l[i] == r[j] {
            i += 1;
            j += 1;
        } else if l[i] < r[j] {
            v.push(l[i].clone());
            i += 1;
        } else {
            j += 1;
        }
    }
    v.append(&mut l[i..].to_vec());
    v
}

pub fn simplify_member(expr: &Expr) -> Expr {
    let u = &expr.operands[0];
    let s = &expr.operands[1];

    if let ExprKind::Set = s.kind {
        if s.num_operands() == 0 {
            Expr::bool(false)
        } else {
            Expr::bool(s.operands.iter().find(|v| *v == u).is_some())
        }
    } else {
        Expr {
            kind: ExprKind::Member,
            operands: vec![u.clone(), s.clone()],
        }
    }
}
