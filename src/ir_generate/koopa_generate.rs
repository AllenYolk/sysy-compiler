use super::exp_solve::ExpSolve;
use super::scopes::*;
use super::temp_symbol::TempSymbolManager;
use crate::ast_generate::ast::*;
use crate::tools::*;

/// Run DFS on the AST and generate the Koopa text.
pub trait KoopaTextGenerate {
    /// Generate the Koopa text recursively.
    ///
    /// `lines` should always be empty when entering the method.
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

        let new_text = format!("fun @{}(){} {{\n{}\n}}", self.ident, ft, b,);
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
            Self::Int => Ok(String::from(": i32")),
            Self::Void => Ok(String::new()),
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
        append_line(lines, "%entry:");
        for item in self.items.iter() {
            let mut s = String::new();
            item.generate(&mut s, scopes, tsm)?;
            append_line(lines, &s);
        }

        Ok(String::new())
    }
}

impl KoopaTextGenerate for BlockItem {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        match self {
            Self::Stmt(stmt) => stmt.generate(lines, scopes, tsm),
            Self::Decl(decl) => decl.generate(lines, scopes, tsm),
        }
    }
}

impl KoopaTextGenerate for Stmt {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        match self {
            Self::Assign(lval, exp) => {
                let id = lval.generate(&mut String::new(), scopes, tsm)?;
                let SymbolTableValue::Var(left) = scopes.get_value(&id)? else {
                    return Err(()); // try to assign a constant
                };

                let mut pre = String::new();
                let right = exp.generate(&mut pre, scopes, tsm)?;
                append_line(lines, &pre);

                let new_line = format!("  store {}, {}", right, left);
                append_line(lines, &new_line);
            }
            Self::Return(exp) => {
                let mut pre = String::new();
                if let Some(expression) = exp {
                    let ret = expression.generate(&mut pre, scopes, tsm)?;
                    append_line(&mut pre, &format!("  ret {}", ret));
                } else {
                    append_line(&mut pre, "  ret");
                }
                append_line(lines, &pre);
            }
            _ => (),
        }

        Ok(String::new())
    }
}

impl KoopaTextGenerate for Decl {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        match self {
            Self::Const(const_decl) => const_decl.generate(lines, scopes, tsm),
            Self::Var(var_decl) => var_decl.generate(lines, scopes, tsm),
        }
    }
}

impl KoopaTextGenerate for ConstDecl {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        for def in self.defs.iter() {
            let mut pre = String::new();
            def.generate(&mut pre, scopes, tsm)?;
            append_line(lines, &pre);
        }

        Ok(String::new())
    }
}

impl KoopaTextGenerate for ConstDef {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        // In SysY, constants are always evaluated at compile time.
        // In the corresponding Koopa code, constants are replaced by their values.
        // So we can just evaluate the constant value and then add the constant to the symbol table.
        // There's no need to generate any Koopa code!
        let mut pre = String::new();
        let init = self.init.generate(&mut pre, scopes, tsm)?;
        append_line(lines, &pre); // `pre` is empty.
        scopes.add_value(&self.ident, &init, true)?;

        Ok(String::new())
    }
}

impl KoopaTextGenerate for ConstInitVal {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        match self {
            Self::Exp(exp) => exp.generate(lines, scopes, tsm),
        }
    }
}

impl KoopaTextGenerate for VarDecl {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        for def in self.defs.iter() {
            let mut pre = String::new();
            def.generate(&mut pre, scopes, tsm)?;
            append_line(lines, &pre);
        }

        Ok(String::new())
    }
}

impl KoopaTextGenerate for VarDef {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        append_line(lines, &format!("  @{} = alloc i32", self.ident));
        scopes.add_value(&self.ident, &format!("@{}", self.ident), false)?;

        if let Some(ref init) = self.init {
            // has initial value
            let mut pre = String::new();
            let init_handle = init.generate(&mut pre, scopes, tsm)?;
            append_line(lines, &pre);
            append_line(lines, &format!("  store {}, @{}", init_handle, self.ident));
        }

        Ok(String::new())
    }
}

impl KoopaTextGenerate for InitVal {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        match self {
            Self::Exp(exp) => exp.generate(lines, scopes, tsm),
        }
    }
}

impl KoopaTextGenerate for ConstExp {
    fn generate(
        &self,
        _lines: &mut String,
        scopes: &mut Scopes,
        _tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        let v = self.solve(scopes)?; // evaluate the constant expression while generating AST.
        Ok(v.to_string()) // return the constant value (as a `String`).
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
            Self::LVal(lval) => {
                let mut pre = String::new();
                let id = lval.generate(&mut pre, scopes, tsm)?;
                append_line(lines, &pre);
                match scopes.get_value(&id)? {
                    SymbolTableValue::Const(s) => {
                        // `s` is a literal value, so we can just return it.
                        Ok(s)
                    }
                    SymbolTableValue::Var(s) => {
                        // `s` is a symbol name pointing to an address, so we need to load the value.
                        let new_temp_symbol = tsm.new_temp_symbol();
                        append_line(lines, &format!("  {} = load {}", new_temp_symbol, s));
                        Ok(new_temp_symbol)
                    }
                }
            }
        }
    }
}

impl KoopaTextGenerate for LVal {
    fn generate(
        &self,
        _lines: &mut String,
        _scopes: &mut Scopes,
        _tsm: &mut TempSymbolManager,
    ) -> Result<String, ()> {
        Ok(self.ident.clone()) // return the identifier
    }
}
