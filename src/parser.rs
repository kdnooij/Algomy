use crate::expression::{Expr, ExprKind};
use crate::pest::Parser;
use pest::iterators::Pair;
use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc, Op, PrattParser},
};

#[derive(Clone, PartialEq, Eq)]
pub struct Assignment {
    pub var: Expr,
    pub val: Expr,
}

pub enum Line {
    Expr(Expr),
    Assignment(Assignment),
    None,
}

#[derive(Parser)]
#[grammar = "alg.pest"]
pub struct AlgomyKernel {
    pratt_parser: PrattParser<Rule>,
    pub assignments: Vec<Assignment>,
}

impl AlgomyKernel {
    pub fn new() -> Self {
        Self {
            pratt_parser: PrattParser::new()
                .op(Op::infix(Rule::add, Assoc::Left)
                    | Op::infix(Rule::sub, Assoc::Left)
                    | Op::infix(Rule::or, Assoc::Left))
                .op(Op::infix(Rule::mul, Assoc::Left)
                    | Op::infix(Rule::div, Assoc::Left)
                    | Op::infix(Rule::and, Assoc::Left))
                .op(Op::infix(Rule::pow, Assoc::Right) | Op::infix(Rule::setdiff, Assoc::Right))
                .op(Op::prefix(Rule::not))
                .op(Op::postfix(Rule::fac))
                .op(Op::prefix(Rule::neg)),
            assignments: Vec::new(),
        }
    }

    /// Parse a program into a Vec of lines represented as Expr.
    pub fn parse_program(&mut self, source: &str) -> Result<Vec<Line>, pest::error::Error<Rule>> {
        let mut ast = vec![];
        let pairs = AlgomyKernel::parse(Rule::program, source)?;
        for pair in pairs {
            match pair.as_rule() {
                Rule::line => {
                    let mut line_pairs = pair.into_inner();
                    let pair = line_pairs.next().unwrap();

                    match pair.as_rule() {
                        Rule::expr => {
                            let line = parse_expr(pair.into_inner(), &self.pratt_parser);
                            ast.push(Line::Expr(line))
                        }
                        Rule::assignment => {
                            let mut pairs = pair.into_inner();
                            let var = parse_symbol(pairs.next().unwrap());
                            let expr = pairs.next().unwrap();
                            let expr = parse_expr(expr.into_inner(), &self.pratt_parser);
                            ast.push(Line::Assignment(Assignment {
                                var: var,
                                val: expr,
                            }))
                        }
                        Rule::EOI => ast.push(Line::None),
                        unknown => panic!("Unknown rule: {:?}", unknown),
                    }
                }
                Rule::EOI => {}
                _ => unreachable!(),
            }
        }
        return Ok(ast);
    }

    /// Parse a program into a Vec of lines represented as Expr.
    pub fn parse_line(&mut self, source: &str) -> Result<Line, pest::error::Error<Rule>> {
        let mut line_pairs = AlgomyKernel::parse(Rule::line, source)?;
        let pair = line_pairs.next().unwrap();

        let line_pair = pair.into_inner().next().unwrap();
        match line_pair.as_rule() {
            Rule::expr => {
                let line = parse_expr(line_pair.into_inner(), &self.pratt_parser);
                Ok(Line::Expr(line))
            }
            Rule::assignment => {
                let mut pairs = line_pair.into_inner();
                let var = parse_symbol(pairs.next().unwrap());
                let expr = pairs.next().unwrap();
                let expr = parse_expr(expr.into_inner(), &self.pratt_parser);
                Ok(Line::Assignment(Assignment { var, val: expr }))
            }
            Rule::EOI => Ok(Line::None),
            unknown => panic!("Unknown rule: {:?}", unknown),
        }
    }
}

fn parse_expr(pairs: Pairs<Rule>, pratt: &PrattParser<Rule>) -> Expr {
    pratt
        .map_primary(|operand| match operand.as_rule() {
            Rule::num => parse_num(operand),
            Rule::symbol => parse_symbol(operand),
            Rule::expr => parse_expr(operand.into_inner(), pratt),
            Rule::func => parse_func(operand.into_inner(), pratt),
            Rule::set => parse_set(operand.into_inner(), pratt),
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
            Rule::not => Expr {
                kind: ExprKind::Not,
                operands: vec![rhs],
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
            Rule::or => Expr {
                kind: ExprKind::Or,
                operands: vec![lhs, rhs],
            },
            Rule::and => Expr {
                kind: ExprKind::And,
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

    // Check for reserved names
    match func_name {
        "Union" => Expr {
            kind: ExprKind::Union,
            operands,
        },
        "Intersection" => Expr {
            kind: ExprKind::Intersection,
            operands,
        },
        "Difference" => Expr {
            kind: ExprKind::Difference,
            operands,
        },
        "Member" => Expr {
            kind: ExprKind::Member,
            operands,
        },
        _ => Expr {
            kind: ExprKind::Func(func_name.to_owned()),
            operands,
        },
    }
}

fn parse_symbol(pair: Pair<Rule>) -> Expr {
    let s = pair.as_str();
    match s {
        "Undefined" => Expr::undefined(),
        "I" => Expr::gaussian(Expr::int(0), Expr::int(1)),
        "True" => Expr::bool(true),
        "False" => Expr::bool(false),
        _ => Expr {
            kind: ExprKind::Symbol(s.to_owned()),
            operands: vec![],
        },
    }
}

fn parse_set(pairs: Pairs<Rule>, pratt: &PrattParser<Rule>) -> Expr {
    let mut operands = vec![];
    for pair in pairs {
        let expr = parse_expr(pair.into_inner(), pratt);
        operands.push(expr);
    }
    Expr {
        kind: ExprKind::Set,
        operands,
    }
}
