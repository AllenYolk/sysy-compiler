use super::scopes::Scopes;
use super::temp_symbol::TempSymbolManager;
use crate::ast_generate::ast::*;
use crate::tools::*;

pub trait KoopaTextGenerate {
    /// lines: always empty when entering the method.
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()>;
}

impl KoopaTextGenerate for CompUnit {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        self.func_def.generate(lines, scopes, tsm)?;
        Ok(String::new())
    }
}

impl KoopaTextGenerate for FuncDef {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        let mut ft_pre = String::new();
        let ft = self.func_type.generate(&mut ft_pre, scopes, tsm)?;
        let mut b = String::new();
        self.block.generate(&mut b, scopes, tsm)?;

        let new_text = format!("fun @{}(): {} {{\n{}\n}}", self.ident, ft, b,);
        append_line(lines, &new_text);

        Ok(String::new())
    }
}

impl KoopaTextGenerate for FuncType {
    fn generate(
        &self,
        _lines: &mut String,
        _scopes: &mut Scopes,
        _tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        match self {
            Self::Int => Ok(String::from("i32")),
            // _ => Err(()),
        }
    }
}

impl KoopaTextGenerate for Block {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        let mut s = String::new();
        self.stmt.generate(&mut s, scopes, tsm)?;
        append_line(lines, &format!("%entry:\n{}", s));

        Ok(String::new())
    }
}

impl KoopaTextGenerate for Stmt {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        let mut pre = String::new();
        let ret = self.exp.generate(&mut pre, scopes, tsm)?;
        append_line(&mut pre, &format!("  ret {}", ret));
        append_line(lines, &pre);

        Ok(String::new())
    }
}

impl KoopaTextGenerate for Exp {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        let mut pre = String::new();
        let var = self.exp.generate(&mut pre, scopes, tsm)?;
        append_line(lines, &pre);
        Ok(var)
    }
}

impl KoopaTextGenerate for LOrExp {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        match self {
            Self::LAnd(exp) => {
                let mut pre = String::new();
                let var = exp.generate(&mut pre, scopes, tsm)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::LOrLAnd(exp1, exp2) => {
                let mut pre1 = String::new();
                let mut pre2 = String::new();
                let var1 = exp1.generate(&mut pre1, scopes, tsm)?;
                let var2 = exp2.generate(&mut pre2, scopes, tsm)?;
                append_line(lines, &pre1);
                append_line(lines, &pre2);

                let new_var = tsm.new_temp_symbol();
                let new_line = format!("  {} = or {}, {}", new_var, var1, var2);
                append_line(lines, &new_line);
                Ok(new_var)
            }
        }
    }
}

impl KoopaTextGenerate for LAndExp {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        match self {
            Self::Eq(exp) => {
                let mut pre = String::new();
                let var = exp.generate(&mut pre, scopes, tsm)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::LAndEq(exp1, exp2) => {
                let mut pre1 = String::new();
                let mut pre2 = String::new();
                let var1 = exp1.generate(&mut pre1, scopes, tsm)?;
                let var2 = exp2.generate(&mut pre2, scopes, tsm)?;
                append_line(lines, &pre1);
                append_line(lines, &pre2);

                let new_var = tsm.new_temp_symbol();
                let new_line = format!("  {} = and {}, {}", new_var, var1, var2);
                append_line(lines, &new_line);
                Ok(new_var)
            }
        }
    }
}

impl KoopaTextGenerate for EqExp {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        match self {
            Self::Rel(exp) => {
                let mut pre = String::new();
                let var = exp.generate(&mut pre, scopes, tsm)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::EqRel(exp1, op, exp2) => {
                let mut pre1 = String::new();
                let mut pre2 = String::new();
                let var1 = exp1.generate(&mut pre1, scopes, tsm)?;
                let var2 = exp2.generate(&mut pre2, scopes, tsm)?;
                append_line(lines, &pre1);
                append_line(lines, &pre2);

                let new_var = tsm.new_temp_symbol();
                let op_str = match *op {
                    EqExpOp::Eq => "eq",
                    EqExpOp::Neq => "ne",
                };
                let new_line = format!("  {} = {} {}, {}", new_var, op_str, var1, var2);
                append_line(lines, &new_line);
                Ok(new_var)
            }
        }
    }
}

impl KoopaTextGenerate for RelExp {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        match self {
            Self::Add(exp) => {
                let mut pre = String::new();
                let var = exp.generate(&mut pre, scopes, tsm)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::RelAdd(exp1, op, exp2) => {
                let mut pre1 = String::new();
                let mut pre2 = String::new();
                let var1 = exp1.generate(&mut pre1, scopes, tsm)?;
                let var2 = exp2.generate(&mut pre2, scopes, tsm)?;
                append_line(lines, &pre1);
                append_line(lines, &pre2);

                let new_var = tsm.new_temp_symbol();
                let op_str = match *op {
                    RelExpOp::Le => "le",
                    RelExpOp::Ge => "ge",
                    RelExpOp::Lt => "lt",
                    RelExpOp::Gt => "gt",
                };
                let new_line = format!("  {} = {} {}, {}", new_var, op_str, var1, var2);
                append_line(lines, &new_line);
                Ok(new_var)
            }
        }
    }
}

impl KoopaTextGenerate for AddExp {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        match self {
            Self::Mul(exp) => {
                let mut pre = String::new();
                let var = exp.generate(&mut pre, scopes, tsm)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::AddMul(exp1, op, exp2) => {
                let mut pre1 = String::new();
                let mut pre2 = String::new();
                let var1 = exp1.generate(&mut pre1, scopes, tsm)?;
                let var2 = exp2.generate(&mut pre2, scopes, tsm)?;
                append_line(lines, &pre1);
                append_line(lines, &pre2);

                let new_var = tsm.new_temp_symbol();
                let op_str = match *op {
                    AddExpOp::Add => "add",
                    AddExpOp::Sub => "sub",
                };
                let new_line = format!("  {} = {} {}, {}", new_var, op_str, var1, var2);
                append_line(lines, &new_line);
                Ok(new_var)
            }
        }
    }
}

impl KoopaTextGenerate for MulExp {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        match self {
            Self::Unary(exp) => {
                let mut pre = String::new();
                let var = exp.generate(&mut pre, scopes, tsm)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::MulUnary(exp1, op, exp2) => {
                let mut pre1 = String::new();
                let mut pre2 = String::new();
                let var1 = exp1.generate(&mut pre1, scopes, tsm)?;
                let var2 = exp2.generate(&mut pre2, scopes, tsm)?;
                append_line(lines, &pre1);
                append_line(lines, &pre2);

                let new_var = tsm.new_temp_symbol();
                let op_str = match *op {
                    MulExpOp::Mul => "mul",
                    MulExpOp::Div => "div",
                    MulExpOp::Mod => "mod",
                };
                let new_line = format!("  {} = {} {}, {}", new_var, op_str, var1, var2);
                append_line(lines, &new_line);
                Ok(new_var)
            }
        }
    }
}

impl KoopaTextGenerate for UnaryExp {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        let mut pre = String::new();
        match self {
            Self::Primary(pexp) => {
                let var = pexp.generate(&mut pre, scopes, tsm)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::Unary(op, uexp) => {
                let var = uexp.generate(&mut pre, scopes, tsm)?;
                append_line(lines, &pre);
                match *op {
                    UnaryExpOp::Pos => Ok(var),
                    UnaryExpOp::Neg => {
                        let new_var = tsm.new_temp_symbol();
                        let new_line = format!("  {} = sub 0, {}", new_var, var);
                        append_line(lines, &new_line);
                        Ok(new_var)
                    }
                    UnaryExpOp::Not => {
                        let new_var = tsm.new_temp_symbol();
                        let new_line = format!("  {} = eq 0, {}", new_var, var);
                        append_line(lines, &new_line);
                        Ok(new_var)
                    }
                }
            }
        }
    }
}

impl KoopaTextGenerate for PrimaryExp {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        match self {
            Self::Exp(exp) => {
                let mut pre = String::new();
                let var = exp.generate(&mut pre, scopes, tsm)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::Num(num) => Ok(format!("{}", num)),
        }
    }
}
