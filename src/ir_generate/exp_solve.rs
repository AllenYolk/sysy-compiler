use super::scopes::*;
use crate::ast_generate::ast::*;

/// Solve the value of an expression.
pub trait ExpSolve {
    /// Evaluate the expression and return its value.
    ///
    /// This method is called when generating AST.
    fn solve(&self, scopes: &Scopes) -> Result<i32, ()>;
}

impl ExpSolve for ConstExp {
    fn solve(&self, scopes: &Scopes) -> Result<i32, ()> {
        self.exp.solve(scopes)
    }
}

impl ExpSolve for Exp {
    fn solve(&self, scopes: &Scopes) -> Result<i32, ()> {
        self.exp.solve(scopes)
    }
}

impl ExpSolve for LOrExp {
    fn solve(&self, scopes: &Scopes) -> Result<i32, ()> {
        match self {
            LOrExp::LAnd(exp) => exp.solve(scopes),
            LOrExp::LOrLAnd(exp1, exp2) => {
                let v1 = exp1.solve(scopes)?;
                let v2 = exp2.solve(scopes)?;
                Ok(((v1 != 0) || (v2 != 0)) as i32)
            }
        }
    }
}

impl ExpSolve for LAndExp {
    fn solve(&self, scopes: &Scopes) -> Result<i32, ()> {
        match self {
            LAndExp::Eq(exp) => exp.solve(scopes),
            LAndExp::LAndEq(exp1, exp2) => {
                let v1 = exp1.solve(scopes)?;
                let v2 = exp2.solve(scopes)?;
                Ok(((v1 != 0) && (v2 != 0)) as i32)
            }
        }
    }
}

impl ExpSolve for EqExp {
    fn solve(&self, scopes: &Scopes) -> Result<i32, ()> {
        match self {
            EqExp::Rel(exp) => exp.solve(scopes),
            EqExp::EqRel(exp1, op, exp2) => {
                let v1 = exp1.solve(scopes)?;
                let v2 = exp2.solve(scopes)?;
                match op {
                    EqExpOp::Eq => Ok((v1 == v2) as i32),
                    EqExpOp::Neq => Ok((v1 != v2) as i32),
                }
            }
        }
    }
}

impl ExpSolve for RelExp {
    fn solve(&self, scopes: &Scopes) -> Result<i32, ()> {
        match self {
            RelExp::Add(exp) => exp.solve(scopes),
            RelExp::RelAdd(exp1, op, exp2) => {
                let v1 = exp1.solve(scopes)?;
                let v2 = exp2.solve(scopes)?;
                match op {
                    RelExpOp::Lt => Ok((v1 < v2) as i32),
                    RelExpOp::Gt => Ok((v1 > v2) as i32),
                    RelExpOp::Le => Ok((v1 <= v2) as i32),
                    RelExpOp::Ge => Ok((v1 >= v2) as i32),
                }
            }
        }
    }
}

impl ExpSolve for AddExp {
    fn solve(&self, scopes: &Scopes) -> Result<i32, ()> {
        match self {
            AddExp::Mul(exp) => exp.solve(scopes),
            AddExp::AddMul(exp1, op, exp2) => {
                let v1 = exp1.solve(scopes)?;
                let v2 = exp2.solve(scopes)?;
                match op {
                    AddExpOp::Add => Ok(v1 + v2),
                    AddExpOp::Sub => Ok(v1 - v2),
                }
            }
        }
    }
}

impl ExpSolve for MulExp {
    fn solve(&self, scopes: &Scopes) -> Result<i32, ()> {
        match self {
            MulExp::Unary(exp) => exp.solve(scopes),
            MulExp::MulUnary(exp1, op, exp2) => {
                let v1 = exp1.solve(scopes)?;
                let v2 = exp2.solve(scopes)?;
                match op {
                    MulExpOp::Mul => Ok(v1 * v2),
                    MulExpOp::Div => Ok(v1 / v2),
                    MulExpOp::Mod => Ok(v1 % v2),
                }
            }
        }
    }
}

impl ExpSolve for UnaryExp {
    fn solve(&self, scopes: &Scopes) -> Result<i32, ()> {
        match self {
            UnaryExp::Primary(exp) => exp.solve(scopes),
            UnaryExp::Unary(op, exp) => {
                let v = exp.solve(scopes)?;
                match op {
                    UnaryExpOp::Pos => Ok(v),
                    UnaryExpOp::Neg => Ok(-v),
                    UnaryExpOp::Not => Ok((v == 0) as i32),
                }
            }
        }
    }
}

impl ExpSolve for PrimaryExp {
    fn solve(&self, scopes: &Scopes) -> Result<i32, ()> {
        match self {
            PrimaryExp::Exp(exp) => exp.solve(scopes),
            PrimaryExp::LVal(lval) => lval.solve(scopes),
            PrimaryExp::Num(num) => Ok(*num),
        }
    }
}

impl ExpSolve for LVal {
    fn solve(&self, scopes: &Scopes) -> Result<i32, ()> {
        let SymbolTableValue::Const(v) = scopes.get_value(&self.ident)? else {
            return Err(());
        };
        i32::from_str_radix(&v, 10).map_err(|_| ())
    }
}
