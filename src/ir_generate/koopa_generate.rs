use super::exp_solve::ExpSolve;
use super::named_symbol::NamedSymbolCounter;
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
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()>;
}

impl KoopaTextGenerate for CompUnit {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        // global values are placed before library function declarations
        for item in self.items.iter() {
            if let CompUnitItem::GlobalDecl(global_decl) = item {
                let mut global_decl_text = String::new();
                global_decl.generate(&mut global_decl_text, scopes, tsm, nsc)?;
                append_line(lines, &global_decl_text);
            }
        }

        // declarations of SysY library functions
        append_line(lines, "decl @getint(): i32\n");
        append_line(lines, "decl @getch(): i32\n");
        append_line(lines, "decl @getarray(*i32): i32\n");
        append_line(lines, "decl @putint(i32)\n");
        append_line(lines, "decl @putch(i32)\n");
        append_line(lines, "decl @putarray(i32, *i32)\n");
        append_line(lines, "decl @starttime()\n");
        append_line(lines, "decl @stoptime()\n");
        // put these functions into the global scope
        scopes.add_function("getint", "@getint", false)?;
        scopes.add_function("getch", "@getch", false)?;
        scopes.add_function("getarray", "@getarray", false)?;
        scopes.add_function("putint", "@putint", true)?;
        scopes.add_function("putch", "@putch", true)?;
        scopes.add_function("putarray", "@putarray", true)?;
        scopes.add_function("starttime", "@starttime", true)?;
        scopes.add_function("stoptime", "@stoptime", true)?;

        // generate function definitions
        for item in self.items.iter() {
            if let CompUnitItem::FuncDef(func_def) = item {
                let mut func_text = String::new();
                func_def.generate(&mut func_text, scopes, tsm, nsc)?;
                append_line(lines, &func_text);
                append_line(lines, " ");
            }
        }
        Ok(String::new())
    }
}

impl KoopaTextGenerate for FuncDef {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        // function return type
        let mut ft = self
            .func_type
            .generate(&mut String::new(), scopes, tsm, nsc)?;
        if !ft.is_empty() {
            ft = format!(": {}", ft);
        }

        // function name
        let func_name = format!("@{}", self.ident);
        scopes.add_function(&self.ident, &func_name, ft.is_empty())?;

        // function parameters
        let mut param_text = String::new();
        let mut func_param_reallocation_text = String::new();
        for (i, param) in self.params.iter().enumerate() {
            let formal_param_symbol = tsm.new_temp_symbol();
            let param_ident = param.generate(&mut String::new(), scopes, tsm, nsc)?;
            let reallocated_param_symbol =
                nsc.inc_and_get_named_symbol(&format!("@{}", param_ident))?;

            if i != 0 {
                param_text.push_str(", ")
            }
            param_text.push_str(&format!("{}: i32", formal_param_symbol));

            append_line(
                &mut func_param_reallocation_text,
                &format!("  {} = alloc i32", reallocated_param_symbol),
            );
            append_line(
                &mut func_param_reallocation_text,
                &format!(
                    "  store {}, {}",
                    formal_param_symbol, reallocated_param_symbol
                ),
            );
            scopes.add_value_to_buffer(&param_ident, &reallocated_param_symbol, false);
        }

        // function body
        let mut body_text = String::new();
        self.block.generate(&mut body_text, scopes, tsm, nsc)?;
        // 1. If there's no `ret` instruction in the function body, we only need to add one at the last line.
        // 2. Only when the return type is `void` can the `ret` instruction be omitted by the original function body.
        let Some(last_line) = body_text.split("\n").last() else {
            return Err(());
        };
        if !last_line.contains("ret") {
            append_line(&mut body_text, "  ret");
        }

        append_line(
            lines,
            &format!("fun {}({}){} {{", func_name, param_text, ft),
        );
        append_line(lines, "%entry:");
        append_line(lines, &func_param_reallocation_text);
        append_line(lines, &body_text);
        append_line(lines, "}");

        Ok(String::new())
    }
}

impl KoopaTextGenerate for FuncType {
    fn generate(
        &self,
        _lines: &mut String,
        _scopes: &mut Scopes,
        _tsm: &mut TempSymbolManager,
        _nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        match self {
            Self::Int => Ok(String::from("i32")),
            Self::Void => Ok(String::new()),
        }
    }
}

impl KoopaTextGenerate for FuncFParam {
    fn generate(
        &self,
        _lines: &mut String,
        _scopes: &mut Scopes,
        _tsm: &mut TempSymbolManager,
        _nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        Ok(self.ident.clone())
    }
}

impl KoopaTextGenerate for Block {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        scopes.enter()?;

        for item in self.items.iter() {
            let mut s = String::new();
            item.generate(&mut s, scopes, tsm, nsc)?;
            append_line(lines, &s);
        }

        scopes.exit();
        Ok(String::new())
    }
}

impl KoopaTextGenerate for BlockItem {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        match self {
            Self::Stmt(stmt) => stmt.generate(lines, scopes, tsm, nsc),
            Self::Decl(decl) => decl.generate(lines, scopes, tsm, nsc),
        }
    }
}

impl KoopaTextGenerate for Stmt {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        match self {
            Self::Assign(lval, exp) => {
                let id = lval.generate(&mut String::new(), scopes, tsm, nsc)?;
                let SymbolTableValue::Var(left) = scopes.get_value(&id)? else {
                    return Err(()); // try to assign a constant
                };

                let mut pre = String::new();
                let right = exp.generate(&mut pre, scopes, tsm, nsc)?;
                append_line(lines, &pre);

                let new_line = format!("  store {}, {}", right, left);
                append_line(lines, &new_line);
            }
            Self::Exp(exp) => {
                if let Some(expression) = exp {
                    let mut pre = String::new();
                    expression.generate(&mut pre, scopes, tsm, nsc)?;
                    append_line(lines, &pre);
                }
            }
            Self::Block(block) => {
                let mut pre = String::new();
                block.generate(&mut pre, scopes, tsm, nsc)?;
                append_line(lines, &pre);
            }
            Self::If(cond, then, otherwise) => {
                // prepare the labels
                let then_label = nsc.inc_and_get_named_symbol("%if_then")?;
                let else_label = nsc.inc_and_get_named_symbol("%if_else")?;
                let end_label = nsc.inc_and_get_named_symbol("%if_end")?;

                // cond generation
                let mut text_for_cond = String::new();
                let cond_handle = cond.generate(&mut text_for_cond, scopes, tsm, nsc)?;
                append_line(lines, &text_for_cond);
                append_line(
                    lines,
                    &format!("  br {}, {}, {}", cond_handle, then_label, else_label),
                );

                // then generation
                append_line(lines, &format!("\n{}:", then_label));
                let mut text_for_then = String::new();
                then.generate(&mut text_for_then, scopes, tsm, nsc)?;
                append_line(lines, &text_for_then);
                append_line(lines, &format!("  jump {}", end_label));

                // else generation
                append_line(lines, &format!("\n{}:", else_label));
                let mut text_for_else = String::new();
                if let Some(otherwise) = otherwise {
                    otherwise.generate(&mut text_for_else, scopes, tsm, nsc)?;
                }
                append_line(lines, &text_for_else);
                append_line(lines, &format!("  jump {}", end_label));

                // end label generation
                append_line(lines, &format!("\n{}:", end_label));
            }
            Self::While(cond, body) => {
                // prepare the labels
                let entry_label = nsc.inc_and_get_named_symbol("%while_entry")?;
                let body_label = nsc.inc_and_get_named_symbol("%while_body")?;
                let end_label = nsc.inc_and_get_named_symbol("%while_end")?;

                // cond generation
                append_line(lines, &format!("  jump {}", entry_label));
                append_line(lines, &format!("\n{}:", entry_label));
                let mut text_for_cond = String::new();
                let cond_handle = cond.generate(&mut text_for_cond, scopes, tsm, nsc)?;
                append_line(lines, &text_for_cond);
                append_line(
                    lines,
                    &format!("  br {}, {}, {}", cond_handle, body_label, end_label),
                );

                // body generation
                append_line(lines, &format!("\n{}:", body_label));
                scopes.enter_loop(&entry_label, &body_label, &end_label);
                let mut text_for_body = String::new();
                body.generate(&mut text_for_body, scopes, tsm, nsc)?;
                scopes.exit_loop();
                append_line(lines, &text_for_body);
                append_line(lines, &format!("  jump {}", entry_label));

                // end label generation
                append_line(lines, &format!("\n{}:", end_label));
            }
            Self::Break => {
                let Ok(LoopLabel{entry: _, body: _, end: end_label}) = scopes.get_cur_loop_labels() else {
                    return Err(());
                };
                append_line(lines, &format!("  jump {}", end_label));

                // The original basic block is splitted into two halves by the `jump` instruction.
                // Hence, we need to add a new label here to indicate the start of a new basic block.
                let new_label = nsc.inc_and_get_named_symbol("%after_break")?;
                append_line(lines, &format!("\n{}:", new_label));
            }
            Self::Continue => {
                let Ok(LoopLabel{entry: entry_label, body: _, end: _}) = scopes.get_cur_loop_labels() else {
                    return Err(());
                };
                append_line(lines, &format!("  jump {}", entry_label));

                // The original basic block is splitted into two halves by the `jump` instruction.
                // Hence, we need to add a new label here to indicate the start of a new basic block.
                let new_label = nsc.inc_and_get_named_symbol("%after_continue")?;
                append_line(lines, &format!("\n{}:", new_label));
            }
            Self::Return(exp) => {
                let mut pre = String::new();
                if let Some(expression) = exp {
                    let ret = expression.generate(&mut pre, scopes, tsm, nsc)?;
                    append_line(&mut pre, &format!("  ret {}", ret));
                } else {
                    append_line(&mut pre, "  ret");
                }
                append_line(lines, &pre);
            }
        }

        Ok(String::new())
    }
}

impl KoopaTextGenerate for GlobalDecl {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        match self.decl {
            // global constant: the same as local ones
            Decl::Const(ref const_decl) => const_decl.generate(lines, scopes, tsm, nsc),
            // global variables: the initial value of a global variable should always be a constant expression.
            // We need to evaluate the rhs and add the value to the global-level symbol table.
            Decl::Var(ref var_decl) => {
                for def in var_decl.defs.iter() {
                    let symbol_name = nsc.inc_and_get_named_symbol(&format!("@{}", &def.ident))?;
                    scopes.add_value(&def.ident, &symbol_name, false)?;
                    let init = match def.init {
                        Some(ref init) => {
                            let InitVal::Exp(exp) = init;
                            exp.solve(scopes)?.to_string()
                        }
                        None => "zeroinit".to_string()
                    };
                    append_line(lines, &format!("global {} = alloc i32, {}", symbol_name, init));
                }
                Ok(String::new())
            },
        }
    }
}

impl KoopaTextGenerate for Decl {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        match self {
            Self::Const(const_decl) => const_decl.generate(lines, scopes, tsm, nsc),
            Self::Var(var_decl) => var_decl.generate(lines, scopes, tsm, nsc),
        }
    }
}

impl KoopaTextGenerate for ConstDecl {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        for def in self.defs.iter() {
            let mut pre = String::new();
            def.generate(&mut pre, scopes, tsm, nsc)?;
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
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        // In SysY, constants are always evaluated at compile time.
        // In the corresponding Koopa code, constants are replaced by their values.
        // So we can just evaluate the constant value and then add the constant to the symbol table.
        // There's no need to generate any Koopa code!
        let mut pre = String::new();
        let init = self.init.generate(&mut pre, scopes, tsm, nsc)?;
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
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        match self {
            Self::Exp(exp) => exp.generate(lines, scopes, tsm, nsc),
        }
    }
}

impl KoopaTextGenerate for VarDecl {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        for def in self.defs.iter() {
            let mut pre = String::new();
            def.generate(&mut pre, scopes, tsm, nsc)?;
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
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        let symbol_name = nsc.inc_and_get_named_symbol(&format!("@{}", &self.ident))?;
        append_line(lines, &format!("  {} = alloc i32", symbol_name));
        scopes.add_value(&self.ident, &symbol_name, false)?;

        if let Some(ref init) = self.init {
            // has initial value
            let mut pre = String::new();
            let init_handle = init.generate(&mut pre, scopes, tsm, nsc)?;
            append_line(lines, &pre);
            append_line(lines, &format!("  store {}, {}", init_handle, symbol_name));
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
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        match self {
            Self::Exp(exp) => exp.generate(lines, scopes, tsm, nsc),
        }
    }
}

impl KoopaTextGenerate for ConstExp {
    fn generate(
        &self,
        _lines: &mut String,
        scopes: &mut Scopes,
        _tsm: &mut TempSymbolManager,
        _nsc: &mut NamedSymbolCounter,
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
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        let mut pre = String::new();
        let var = self.exp.generate(&mut pre, scopes, tsm, nsc)?;
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
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        match self {
            Self::LAnd(exp) => {
                let mut pre = String::new();
                let var = exp.generate(&mut pre, scopes, tsm, nsc)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::LOrLAnd(exp1, exp2) => {
                // Prepare the labels
                let rhs_label = nsc.inc_and_get_named_symbol("%or_rhs")?;
                let end_label = nsc.inc_and_get_named_symbol("%or_end")?;

                // Koopa has "SSA" feature, so we have to allocate a memory slot to store the result of or operation.
                // Since the result is actually a temporary value, we don't need to add it to the symbol table.
                let result_name = nsc.inc_and_get_named_symbol("%or")?;
                append_line(lines, &format!("  {} = alloc i32", result_name));

                // left-hand side
                let mut pre1 = String::new();
                let var1 = exp1.generate(&mut pre1, scopes, tsm, nsc)?;
                append_line(lines, &pre1);
                let lvar1 = tsm.new_temp_symbol();
                append_line(lines, &format!("  {} = ne {}, 0", lvar1, var1));
                append_line(lines, &format!("  store {}, {}", lvar1, result_name));
                append_line(
                    lines,
                    &format!("  br {}, {}, {}", lvar1, end_label, rhs_label),
                );

                // right-hand side
                append_line(lines, &format!("\n{}:", rhs_label));
                let mut pre2 = String::new();
                let var2 = exp2.generate(&mut pre2, scopes, tsm, nsc)?;
                append_line(lines, &pre2);
                let lvar2 = tsm.new_temp_symbol();
                append_line(lines, &format!("  {} = ne {}, 0", lvar2, var2));
                append_line(lines, &format!("  store {}, {}", lvar2, result_name));
                append_line(lines, &format!("  jump {}", end_label));

                // end
                append_line(lines, &format!("\n{}:", end_label));
                let new_var = tsm.new_temp_symbol();
                append_line(lines, &format!("  {} = load {}", new_var, result_name));
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
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        match self {
            Self::Eq(exp) => {
                let mut pre = String::new();
                let var = exp.generate(&mut pre, scopes, tsm, nsc)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::LAndEq(exp1, exp2) => {
                // Prepare the labels
                let rhs_label = nsc.inc_and_get_named_symbol("%and_rhs")?;
                let end_label = nsc.inc_and_get_named_symbol("%and_end")?;

                // Koopa has "SSA" feature, so we have to allocate a memory slot to store the result of or operation.
                // Since the result is actually a temporary value, we don't need to add it to the symbol table.
                let result_name = nsc.inc_and_get_named_symbol("%and")?;
                append_line(lines, &format!("  {} = alloc i32", result_name));

                // left-hand side
                let mut pre1 = String::new();
                let var1 = exp1.generate(&mut pre1, scopes, tsm, nsc)?;
                append_line(lines, &pre1);
                let lvar1 = tsm.new_temp_symbol();
                append_line(lines, &format!("  {} = ne {}, 0", lvar1, var1));
                append_line(lines, &format!("  store {}, {}", lvar1, result_name));
                append_line(
                    lines,
                    &format!("  br {}, {}, {}", lvar1, rhs_label, end_label),
                );

                // right-hand side
                append_line(lines, &format!("\n{}:", rhs_label));
                let mut pre2 = String::new();
                let var2 = exp2.generate(&mut pre2, scopes, tsm, nsc)?;
                append_line(lines, &pre2);
                let lvar2 = tsm.new_temp_symbol();
                append_line(lines, &format!("  {} = ne {}, 0", lvar2, var2));
                append_line(lines, &format!("  store {}, {}", lvar2, result_name));
                append_line(lines, &format!("  jump {}", end_label));

                // end
                append_line(lines, &format!("\n{}:", end_label));
                let new_var = tsm.new_temp_symbol();
                append_line(lines, &format!("  {} = load {}", new_var, result_name));
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
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        match self {
            Self::Rel(exp) => {
                let mut pre = String::new();
                let var = exp.generate(&mut pre, scopes, tsm, nsc)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::EqRel(exp1, op, exp2) => {
                let mut pre1 = String::new();
                let mut pre2 = String::new();
                let var1 = exp1.generate(&mut pre1, scopes, tsm, nsc)?;
                let var2 = exp2.generate(&mut pre2, scopes, tsm, nsc)?;
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
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        match self {
            Self::Add(exp) => {
                let mut pre = String::new();
                let var = exp.generate(&mut pre, scopes, tsm, nsc)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::RelAdd(exp1, op, exp2) => {
                let mut pre1 = String::new();
                let mut pre2 = String::new();
                let var1 = exp1.generate(&mut pre1, scopes, tsm, nsc)?;
                let var2 = exp2.generate(&mut pre2, scopes, tsm, nsc)?;
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
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        match self {
            Self::Mul(exp) => {
                let mut pre = String::new();
                let var = exp.generate(&mut pre, scopes, tsm, nsc)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::AddMul(exp1, op, exp2) => {
                let mut pre1 = String::new();
                let mut pre2 = String::new();
                let var1 = exp1.generate(&mut pre1, scopes, tsm, nsc)?;
                let var2 = exp2.generate(&mut pre2, scopes, tsm, nsc)?;
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
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        match self {
            Self::Unary(exp) => {
                let mut pre = String::new();
                let var = exp.generate(&mut pre, scopes, tsm, nsc)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::MulUnary(exp1, op, exp2) => {
                let mut pre1 = String::new();
                let mut pre2 = String::new();
                let var1 = exp1.generate(&mut pre1, scopes, tsm, nsc)?;
                let var2 = exp2.generate(&mut pre2, scopes, tsm, nsc)?;
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
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        let mut pre = String::new();
        match self {
            Self::Primary(pexp) => {
                let var = pexp.generate(&mut pre, scopes, tsm, nsc)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::FuncCall(ident, params) => {
                let FunctionInfo {
                    symbol: func_symbol,
                    return_void,
                } = scopes.get_function(&ident)?;

                let mut param_text = String::new();
                for (i, param) in params.iter().enumerate() {
                    let mut param_generation_text = String::new();
                    let param_var = param.generate(&mut param_generation_text, scopes, tsm, nsc)?;
                    append_line(lines, &param_generation_text);
                    if i > 0 {
                        param_text.push_str(", ");
                    }
                    param_text.push_str(&param_var);
                }

                if return_void {
                    append_line(lines, &format!("  call {}({})", func_symbol, param_text));
                    Ok(String::new())
                } else {
                    let result_symbol = tsm.new_temp_symbol();
                    append_line(
                        lines,
                        &format!("  {} = call {}({})", result_symbol, func_symbol, param_text),
                    );
                    Ok(result_symbol)
                }
            }
            Self::Unary(op, uexp) => {
                let var = uexp.generate(&mut pre, scopes, tsm, nsc)?;
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
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        match self {
            Self::Exp(exp) => {
                let mut pre = String::new();
                let var = exp.generate(&mut pre, scopes, tsm, nsc)?;
                append_line(lines, &pre);
                Ok(var)
            }
            Self::Num(num) => Ok(format!("{}", num)),
            Self::LVal(lval) => {
                let mut pre = String::new();
                let id = lval.generate(&mut pre, scopes, tsm, nsc)?;
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
        _nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        Ok(self.ident.clone()) // return the identifier
    }
}
