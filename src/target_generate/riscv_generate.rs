use super::context::*;
use super::function_scan::*;
use super::value_location::*;
use crate::tools::*;
use koopa::ir::entities::*;
use koopa::ir::*;

/// Generate RISC-V code from the given Koopa object.
pub trait RiscvGenerate {
    /// The return type of the method `generate`.
    type Ret;

    /// Generate RISC-V code.
    ///
    /// `lines` should always be empty when entering the method.
    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()>;
}

impl RiscvGenerate for Program {
    type Ret = ();

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        // global variables
        let mut first_time = true;
        for &val in self.inst_layout() {
            let val_data = self.borrow_value(val);
            if let Some(val_name) = val_data.name() {
                if first_time {
                    append_line(lines, "  .data");
                    first_time = false;
                }
                append_line(lines, &format!("  .globl {}", val_name));
            }
        }

        // function definitions
        for &func in self.func_layout() {
            // First, scan the function in order to get some information.
            // 1. The addresses of the temporary variables.
            // 2. The size of the stack frame.
            // 3. The address of the return address slot.
            // Store the information in `cxt.func`.
            let func_data = self.func(func);
            cxt.func = Some(FunctionScanResult::try_from(func, func_data)?);

            // Then, generate the instructions in the function body.
            let mut new_lines = String::new();
            func_data.generate(&mut new_lines, cxt)?;

            append_line(lines, &new_lines);
            append_line(lines, " ");
        }

        Ok(())
    }
}

impl RiscvGenerate for FunctionData {
    type Ret = ();

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        let func_name = &self.name()[1..];
        let mut name_lines = String::new();
        append_line(&mut name_lines, "  .text");
        append_line(&mut name_lines, &format!("  .globl {}", func_name));
        append_line(&mut name_lines, &format!("{}:", func_name));

        let mut body_lines = String::new();
        for (bb, node) in self.layout().bbs() {
            // get basic block name
            let Some(bbd) = cxt.get_basic_block_data(*bb) else {
                return Err(());
            };
            let &Some(ref bb_name) = bbd.name() else {
                return Err(());
            };
            append_line(
                &mut body_lines,
                &format!("{}:", bb_name.replace("%", "").replace("@", "")),
            );

            // generate basic block instructions
            for &inst_val in node.insts().keys() {
                let inst_val_data = self.dfg().value(inst_val); // an Koopa instruction
                let mut new_lines = String::new();
                let loc = inst_val_data.generate(&mut new_lines, cxt)?; // the location of the instruction's left-hand side

                match loc {
                    ValueLocation::PlaceHolder(p) => {
                        let Some(real_loc) = cxt.get_value_location(inst_val) else{
                            return Err(());
                        };
                        let real_loc_str = match real_loc {
                            ValueLocation::Imm(s) => s,
                            ValueLocation::Reg(s) => s,
                            ValueLocation::Stack(s) => s,
                            _ => return Err(()),
                        };
                        new_lines = new_lines.replace(&p, &real_loc_str);
                    }
                    _ => (),
                }

                append_line(&mut body_lines, &new_lines);
            }
        }

        // add prologue and epilogue
        append_line(lines, &name_lines);
        let Some(ref func_info) = cxt.func else {
            return Err(());
        };
        let sp_shift = func_info.stack_frame_size;

        let mut pro = String::new();
        let mut epi = String::new();
        if sp_shift > 0 {
            append_line(&mut pro, &format!("  addi sp, sp, -{}", sp_shift));
        }
        if let Some(ref ra_loc) = func_info.ra_slot_location {
            // there's a `call` in the function body, and we need to save the `ra` register
            if let ValueLocation::Stack(ref ra_addr) = ra_loc {
                append_line(&mut pro, &format!("  sw ra, {}", ra_addr));
                append_line(&mut epi, &format!("  lw ra, {}", ra_addr));
            } else {
                return Err(());
            }
        }
        if sp_shift > 0 {
            append_line(&mut epi, &format!("  addi sp, sp, {}", sp_shift));
        }
        if pro.is_empty() {
            pro = "  # no prologue".to_string();
        }
        if epi.is_empty() {
            epi = "  # no epilogue".to_string();
        }
        append_line(lines, &pro);
        body_lines = body_lines.replace("<epilogue>", &epi);
        append_line(lines, &body_lines);

        Ok(())
    }
}

impl RiscvGenerate for Value {
    type Ret = ValueLocation;

    /// Find the location of the `Value`.
    ///
    /// Search in the HashMap `cxt.value_locations` by calling `cxt.get_value_location(..)`.
    fn generate(&self, _lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        let Some(value_data) = cxt.get_value_data(*self) else {
            return Err(());
        };

        match value_data.kind() {
            ValueKind::Integer(val) => Ok(ValueLocation::Imm(format!("{}", val.value()))),
            _ => {
                if let Some(loc) = cxt.get_value_location(*self) {
                    Ok(loc.clone())
                } else {
                    Err(())
                }
            }
        }
    }
}

impl RiscvGenerate for ValueData {
    type Ret = ValueLocation;

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        match self.kind() {
            // integer constant
            ValueKind::Integer(val) => val.generate(lines, cxt),
            // allocation operation
            ValueKind::Alloc(val) => val.generate(lines, cxt),
            // load operation
            ValueKind::Load(val) => val.generate(lines, cxt),
            // store operation
            ValueKind::Store(val) => val.generate(lines, cxt),
            // binary operation
            ValueKind::Binary(val) => val.generate(lines, cxt),
            // branch operation
            ValueKind::Branch(val) => val.generate(lines, cxt),
            // jump operation
            ValueKind::Jump(val) => val.generate(lines, cxt),
            // function call
            ValueKind::Call(val) => val.generate(lines, cxt),
            // function return
            ValueKind::Return(val) => val.generate(lines, cxt),
            // others
            _ => Ok(ValueLocation::None),
        }
    }
}

impl RiscvGenerate for values::Integer {
    type Ret = ValueLocation;

    fn generate(&self, _lines: &mut String, _cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        Err(())
    }
}

impl RiscvGenerate for values::Alloc {
    type Ret = ValueLocation;

    fn generate(&self, _lines: &mut String, _cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        Ok(ValueLocation::None)
    }
}

impl RiscvGenerate for values::Load {
    type Ret = ValueLocation;

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        let src = self.src().generate(&mut String::new(), cxt)?;
        append_line(lines, &src.move_to_reg("t0"));
        append_line(lines, "  sw t0, <tar>");

        Ok(ValueLocation::PlaceHolder("<tar>".to_string()))
    }
}

impl RiscvGenerate for values::Store {
    type Ret = ValueLocation;

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        let val = self.value().generate(&mut String::new(), cxt)?;
        let dest = self.dest().generate(&mut String::new(), cxt)?;
        if let ValueLocation::Stack(_) = dest {
            append_line(lines, &val.move_to(dest.clone()));
        } else {
            return Err(());
        }

        Ok(ValueLocation::None)
    }
}

impl RiscvGenerate for values::Binary {
    type Ret = ValueLocation;

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        let loc_l = self.lhs().generate(&mut String::new(), cxt)?;
        let loc_r = self.rhs().generate(&mut String::new(), cxt)?;

        append_line(lines, &loc_l.move_to_reg("t0"));
        append_line(lines, &loc_r.move_to_reg("t1"));
        match self.op() {
            BinaryOp::NotEq => {
                append_line(lines, "  xor t0, t0, t1");
                append_line(lines, "  snez t0, t0");
            }
            BinaryOp::Eq => {
                append_line(lines, "  xor t0, t0, t1");
                append_line(lines, "  seqz t0, t0");
            }
            BinaryOp::Ge => {
                append_line(lines, "  slt t0, t0, t1");
                append_line(lines, "  seqz t0, t0");
            }
            BinaryOp::Le => {
                append_line(lines, "  sgt t0, t0, t1");
                append_line(lines, "  seqz, t0, t0");
            }
            op @ _ => {
                let verb = match op {
                    BinaryOp::Gt => "sgt",
                    BinaryOp::Lt => "slt",
                    BinaryOp::Add => "add",
                    BinaryOp::Sub => "sub",
                    BinaryOp::Mul => "mul",
                    BinaryOp::Div => "div",
                    BinaryOp::Mod => "rem",
                    BinaryOp::And => "and",
                    BinaryOp::Or => "or",
                    BinaryOp::Xor => "xor",
                    BinaryOp::Shl => "sll",
                    BinaryOp::Shr => "srl",
                    BinaryOp::Sar => "sra",
                    _ => return Err(()),
                };
                append_line(lines, &format!("  {} t0, t0, t1", verb));
            }
        };
        append_line(lines, "  sw t0, <tar>");

        Ok(ValueLocation::PlaceHolder("<tar>".to_string()))
    }
}

impl RiscvGenerate for values::Branch {
    type Ret = ValueLocation;

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        let cond_value = self.cond();
        let cond_loc = cond_value.generate(&mut String::new(), cxt)?;
        append_line(lines, &cond_loc.move_to_reg("t0"));

        // look up basic block names
        let Some(true_bb_data) = cxt.get_basic_block_data(self.true_bb()) else {
            return Err(());
        };
        let Some(false_bb_data) = cxt.get_basic_block_data(self.false_bb()) else {
            return Err(());
        };
        let Some(true_bb_name) = true_bb_data.name() else {
            return Err(());
        };
        let Some(false_bb_name) = false_bb_data.name() else {
            return Err(());
        };

        append_line(
            lines,
            &format!(
                "  beqz t0, {}",
                true_bb_name.replace("%", "").replace("@", "")
            ),
        );
        append_line(
            lines,
            &format!("  j {}", false_bb_name.replace("%", "").replace("@", "")),
        );

        Ok(ValueLocation::None)
    }
}

impl RiscvGenerate for values::Jump {
    type Ret = ValueLocation;

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        let to_bb = self.target();
        let Some(bb_data) = cxt.get_basic_block_data(to_bb) else {
            return Err(());
        };
        let Some(bb_name) = bb_data.name() else {
            return Err(());
        };
        append_line(
            lines,
            &format!("  j {}", bb_name.replace("%", "").replace("@", "")),
        );

        Ok(ValueLocation::None)
    }
}

impl RiscvGenerate for values::Call {
    type Ret = ValueLocation;

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        // Since all the temporary variables are stored on the stack frame,
        // we don't need to save the caller-saved registers!!!
        // Nice!

        // Prepare the arguments.
        let args = self.args();
        let Some(stack_frame_size) = cxt.get_current_stack_frame_size() else {
            return Err(());
        };
        for (i, arg) in args.iter().enumerate() {
            let loc = arg.generate(&mut String::new(), cxt)?;
            append_line(lines, &loc.act_as_function_arg(i, stack_frame_size));
        }

        // Call the function.
        let callee = self.callee();
        let callee_data = cxt.get_function_data(callee);
        append_line(lines, &format!("  call {}", &callee_data.name()[1..]));

        // Get the return value.
        if format!("{:?}", callee_data.ty()) == "()".to_string() {
            println!("unit return!");
            Ok(ValueLocation::None)
        } else {
            println!("i32 return!");
            append_line(lines, "  sw a0, <tar>");
            Ok(ValueLocation::PlaceHolder("<tar>".to_string()))
        }
    }
}

impl RiscvGenerate for values::Return {
    type Ret = ValueLocation;

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        if let Some(ret_val) = self.value() {
            let loc = ret_val.generate(&mut String::new(), cxt)?;
            append_line(lines, &loc.move_to_reg("a0"));
        }
        append_line(lines, "<epilogue>"); // a place holder, which will be replaced by the epilogue in `FunctionData.generate`.
        append_line(lines, "  ret");

        Ok(ValueLocation::None)
    }
}
