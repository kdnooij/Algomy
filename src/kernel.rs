use crate::{parser::{AlgomyKernel, Line, Assignment}, expression::Expr, simplify};

impl AlgomyKernel {
    pub fn evaluate_line(&mut self, line: Line) -> Option<Expr> {
        match line {
            Line::Expr(mut expr) => {
                for Assignment { var, val } in self.assignments.iter() {
                    expr = expr.substitute(var, val);
                }
                Some(simplify(&expr))
            }
            Line::Assignment(Assignment { var, val }) => {
                self.add_assignment(var, val);
                None
            }
            Line::None => None,
        }
    }

    #[allow(unused)]
    pub fn parse_eval_line(&mut self, source: &str) -> Expr {
        let line = self.parse_line(source).unwrap();
        self.evaluate_line(line).unwrap()
    }

    fn add_assignment(&mut self, var: Expr, val: Expr) {
        let idx = self
            .assignments
            .iter()
            .enumerate()
            .find(|(_, a)| a.var == var);
        if let Some((i, _)) = idx {
            self.assignments[i].val = val;
        } else {
            self.assignments.push(Assignment {
                var: var.clone(),
                val,
            });
        }
    }

    pub fn clear_session(&mut self) {
        self.assignments.clear();
    }

    pub fn clear_variable(&mut self, var: &Expr) {
        let idx = self
            .assignments
            .iter()
            .enumerate()
            .find(|(_, a)| a.var == *var);
        if let Some((i, _)) = idx {
            self.assignments.remove(i);
        }
    }
}
