use super::scopes::Scopes;
use super::tempvar::TempSymbolManager;
use crate::astgen::ast::*;
use crate::tools::*;

pub trait KoopaTextGenerate {
    /// lines: always empty when entering the method.
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tvm: &mut TempSymbolManager,
    ) -> Result<String, ()>;
}

impl KoopaTextGenerate for CompUnit {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tvm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        self.func_def.generate(lines, scopes, tvm)?;
        Ok(String::new())
    }
}

impl KoopaTextGenerate for FuncDef {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tvm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        let ft = self.func_type.generate(lines, scopes, tvm)?;
        let mut b = String::new();
        self.block.generate(&mut b, scopes, tvm)?;

        let new_text = format!("fun @{}(): {} {{\n{}\n}}", self.ident, ft, b,);
        lines.push_str(&new_text);

        Ok(String::new())
    }
}

impl KoopaTextGenerate for FuncType {
    fn generate(
        &self,
        _lines: &mut String,
        _scopes: &mut Scopes,
        _tvm: &mut TempSymbolManager,
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
        tvm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        let mut s = String::new();
        self.stmt.generate(&mut s, scopes, tvm)?;
        lines.push_str(&format!("%entry:\n{}", s));

        Ok(String::new())
    }
}

impl KoopaTextGenerate for Stmt {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tvm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        let mut pre = String::new();
        let ret = self.exp.generate(&mut pre, scopes, tvm)?;
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
        tvm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        let mut pre = String::new();
        let var = self.exp.generate(&mut pre, scopes, tvm)?;
        lines.push_str(&pre);
        Ok(var)
    }
}

impl KoopaTextGenerate for LOrExp {
    fn generate(
        &self,
        _lines: &mut String,
        _scopes: &mut Scopes,
        _tvm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        Ok("".into())
    }
}

impl KoopaTextGenerate for LAndExp {
    fn generate(
        &self,
        _lines: &mut String,
        _scopes: &mut Scopes,
        _tvm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        Ok("".into())
    }
}

impl KoopaTextGenerate for EqExp {
    fn generate(
        &self,
        _lines: &mut String,
        _scopes: &mut Scopes,
        _tvm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        Ok("".into())
    }
}

impl KoopaTextGenerate for RelExp {
    fn generate(
        &self,
        _lines: &mut String,
        _scopes: &mut Scopes,
        _tvm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        Ok("".into())
    }
}

impl KoopaTextGenerate for AddExp {
    fn generate(
        &self,
        _lines: &mut String,
        _scopes: &mut Scopes,
        _tvm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        Ok("".into())
    }
}

impl KoopaTextGenerate for MulExp {
    fn generate(
        &self,
        _lines: &mut String,
        _scopes: &mut Scopes,
        _tvm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        Ok("".into())
    }
}

impl KoopaTextGenerate for UnaryExp {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tvm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        let mut pre = String::new();
        match self {
            Self::Primary(pexp) => {
                let var = pexp.generate(&mut pre, scopes, tvm)?;
                lines.push_str(&pre);
                Ok(var)
            }
            Self::Unary(op, uexp) => {
                let var = uexp.generate(&mut pre, scopes, tvm)?;
                lines.push_str(&pre);
                match *op {
                    UnaryExpOp::Pos => Ok(var),
                    UnaryExpOp::Neg => {
                        let new_var = tvm.new_temp_symbol();
                        let new_line = format!("  {} = sub 0, {}", new_var, var);
                        append_line(lines, &new_line);
                        Ok(new_var)
                    }
                    UnaryExpOp::Not => {
                        let new_var = tvm.new_temp_symbol();
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
        tvm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        match self {
            Self::Exp(exp) => {
                let mut pre = String::new();
                let var = exp.generate(&mut pre, scopes, tvm)?;
                lines.push_str(&pre);
                Ok(var)
            }
            Self::Num(num) => Ok(format!("{}", num)),
        }
    }
}
