use super::context::*;
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
        // global values
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

        // function names
        first_time = true;
        for &func in self.func_layout() {
            let func_name = &self.func(func).name()[1..];
            if first_time {
                append_line(lines, "  .text");
                first_time = false;
            }
            append_line(lines, &format!("  .globl {}", func_name));
        }

        // function definitions
        for &func in self.func_layout() {
            cxt.func = Some(func);
            cxt.reset_offset();
            let func_data = self.func(func);
            let mut new_lines = String::new();
            func_data.generate(&mut new_lines, cxt)?;
            append_line(lines, &new_lines);
        }

        Ok(())
    }
}

impl RiscvGenerate for FunctionData {
    type Ret = ();

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        let mut name_lines = String::new();
        let func_name = &self.name()[1..];
        append_line(&mut name_lines, &format!("{}:", func_name));

        let mut body_lines = String::new();
        for (_bb, node) in self.layout().bbs() {
            for &inst_val in node.insts().keys() {
                let inst_val_data = self.dfg().value(inst_val); // an Koopa instruction
                let mut new_lines = String::new();
                let loc = inst_val_data.generate(&mut new_lines, cxt)?; // the location of the instruction's left-hand side
                append_line(&mut body_lines, &new_lines);
                match loc {
                    ValueLocation::None => (),
                    _ => {
                        cxt.set_value_location(inst_val, loc);
                    }
                }
            }
        }

        // add prologue and epilogue
        append_line(lines, &name_lines);
        let mut pro = String::from("  # no prologue");
        let mut epi = String::from("  # no epilogue");
        let sp_shift = cxt.total_offset();
        if sp_shift > 0 {
            pro = format!("  addi sp, sp, -{}", sp_shift);
            epi = format!("  addi sp, sp, {}", sp_shift);
        }
        append_line(lines, &pro);
        body_lines = body_lines.replace("<epilogue>", &epi);
        append_line(lines, &body_lines);

        Ok(())
    }
}

impl RiscvGenerate for Value {
    type Ret = ValueLocation;

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

    fn generate(&self, _lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        let offset = cxt.alloc_local_stack_variable(4);
        Ok(ValueLocation::Stack(format!("{}(sp)", offset)))
    }
}

impl RiscvGenerate for values::Load {
    type Ret = ValueLocation;

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        let src = self.src().generate(&mut String::new(), cxt)?;
        append_line(lines, &src.move_to_reg("t0"));
        let offset = cxt.alloc_local_stack_variable(4);
        append_line(lines, &format!("  sw t0, {}(sp)", offset));

        Ok(ValueLocation::Stack(format!("{}(sp)", offset)))
    }
}

impl RiscvGenerate for values::Store {
    type Ret = ValueLocation;

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        let val = self.value().generate(&mut String::new(), cxt)?;
        append_line(lines, &val.move_to_reg("t0"));

        let dest = self.dest().generate(&mut String::new(), cxt)?;
        if let ValueLocation::Stack(dest_loc) = dest {
            append_line(lines, &format!("  sw t0, {}", dest_loc));
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
        let offset = cxt.alloc_local_stack_variable(4);
        append_line(lines, &format!("  sw t0, {}(sp)", offset));

        Ok(ValueLocation::Stack(format!("{}(sp)", offset)))
    }
}

impl RiscvGenerate for values::Return {
    type Ret = ValueLocation;

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        if let Some(ret_val) = self.value() {
            let loc = ret_val.generate(&mut String::new(), cxt)?;
            append_line(lines, &loc.move_to_reg("a0"));
        }
        append_line(lines, "<epilogue>");
        append_line(lines, "  ret");

        Ok(ValueLocation::None)
    }
}
