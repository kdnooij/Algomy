use crate::expression::{Expr, ExprKind};
use crate::pest::Parser;
use lazy_static::lazy_static;
use pest::iterators::Pair;
use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc, Op, PrattParser},
};

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        // Create a precedence climber where operations have the following precedence:
        // [+, -] < [*, /] < [^] < [!] < [(-)]
        PrattParser::new().op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
        .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
        .op(Op::infix(Rule::pow, Assoc::Right))
        .op(Op::postfix(Rule::fac))
        .op(Op::prefix(Rule::neg))
    };
}

#[derive(Parser)]
#[grammar = "alg.pest"]
pub struct AlgParser;

/// Parse a string into an expression.
pub fn parse(source: &str) -> Result<Vec<Expr>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = AlgParser::parse(Rule::program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::line => {
                let mut line_pairs = pair.into_inner();
                let pair = line_pairs.next().unwrap();

                match pair.as_rule() {
                    Rule::expr => {
                        let line = parse_expr(pair.into_inner(), &PRATT_PARSER);
                        ast.push(line);
                    }
                    Rule::EOI => (),
                    unknown => panic!("Unknown rule: {:?}", unknown),
                }
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }
    return Ok(ast);
}

fn parse_expr(pairs: Pairs<Rule>, pratt: &PrattParser<Rule>) -> Expr {
    pratt
        .map_primary(|operand| match operand.as_rule() {
            Rule::num => parse_num(operand),
            Rule::symbol => parse_symbol(operand),
            Rule::expr => parse_expr(operand.into_inner(), pratt),
            Rule::func => parse_func(operand.into_inner(), pratt),
            _ => unreachable!(),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            // Change -a -> (-1)*a
            Rule::neg => Expr {
                kind: ExprKind::Product,
                operands: vec![
                    Expr {
                        kind: ExprKind::Integer(-1),
                        operands: vec![],
                    },
                    rhs,
                ],
            },
            _ => unreachable!(),
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::fac => Expr {
                kind: ExprKind::Factorial,
                operands: vec![lhs],
            },
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => Expr {
                kind: ExprKind::Sum,
                operands: vec![lhs, rhs],
            },
            Rule::sub => Expr {
                kind: ExprKind::Difference,
                operands: vec![lhs, rhs],
            },
            Rule::mul => Expr {
                kind: ExprKind::Product,
                operands: vec![lhs, rhs],
            },
            Rule::div => Expr {
                kind: ExprKind::Quotient,
                operands: vec![lhs, rhs],
            },
            Rule::pow => Expr {
                kind: ExprKind::Power,
                operands: vec![lhs, rhs],
            },
            _ => unreachable!(),
        })
        .parse(pairs)
}

fn parse_num(pair: Pair<Rule>) -> Expr {
    if let Ok(n) = pair.as_str().parse::<i32>() {
        return Expr {
            kind: ExprKind::Integer(n as i64),
            operands: vec![],
        };
    };
    // if let Ok(n) = pair.as_str().parse::<i32>() {
    //     return Expr {
    //         kind: ExprKind::Float(n as i64),
    //         operands: vec![],
    //     };
    // };
    panic!("Invalid number: {:?}", pair.as_str());
}

fn parse_func(mut pairs: Pairs<Rule>, pratt: &PrattParser<Rule>) -> Expr {
    let func_name = pairs.next().unwrap().as_str();
    let mut operands = vec![];
    for pair in pairs {
        let expr = parse_expr(pair.into_inner(), pratt);
        operands.push(expr);
    }
    Expr {
        kind: ExprKind::Func(func_name.to_owned()),
        operands,
    }
}

fn parse_symbol(pair: Pair<Rule>) -> Expr {
    let s = pair.as_str();
    match s {
        "I" => Expr::complex(Expr::int(0), Expr::int(1)),
        _ => Expr {
            kind: ExprKind::Symbol(s.to_owned()),
            operands: vec![],
        },
    }
}
