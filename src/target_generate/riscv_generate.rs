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
        Type::set_ptr_size(4); // necessary according to the tutorial

        // global variables
        for &val in self.inst_layout() {
            let val_data = self.borrow_value(val);
            let Some(val_name) = val_data.name() else { // with "@" or "%" prefix
                return Err(());
            };
            // store the location of the global variable
            cxt.add_global_value(val, ValueLocation::Global(String::from(&val_name[1..])))?;

            let mut initialization_line = String::new();
            // call `generate` on `GlobalAlloc` to generate the initialization line
            val_data.generate(&mut initialization_line, cxt)?;

            // append the code lines to `lines`
            append_line(lines, &format!("  .data"));
            append_line(lines, &format!("  .globl {}", &val_name[1..]));
            append_line(lines, &format!("{}:", &val_name[1..]));
            append_line(lines, &initialization_line);
            append_line(lines, " ");   
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
            if !new_lines.is_empty() {
                append_line(lines, " ");
            }
        }

        Ok(())
    }
}

impl RiscvGenerate for FunctionData {
    type Ret = ();

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        // skip the function if it's a declaration (rather than a definition)
        if let None = self.layout().entry_bb() {
            return Ok(());
        }

        let func_name = &self.name()[1..];
        let mut name_lines = String::new();
        append_line(&mut name_lines, "  .text");
        append_line(&mut name_lines, &format!("  .globl {}", func_name));
        append_line(&mut name_lines, &format!("{}:", func_name));

        let mut body_lines = String::new();
        for (bb, node) in self.layout().bbs() {
            // get basic block name
            let Some(bbd) = cxt.get_basic_block_data_in_current_function(*bb) else {
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
                        let Some(real_loc) = cxt.get_value_location_local_or_global(inst_val) else{
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
        let Some(value_data) = cxt.get_value_data_locally_or_globally(*self) else {
            return Err(());
        };

        match value_data.kind() {
            ValueKind::Integer(val) => Ok(ValueLocation::Imm(format!("{}", val.value()))),
            _ => {
                if let Some(loc) = cxt.get_value_location_local_or_global(*self) {
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
            //////////////////////////////////////////////////////////////////////////////////////////////
            // initialization of a global value                                                         //
            //////////////////////////////////////////////////////////////////////////////////////////////

            // integer constant
            ValueKind::Integer(val) => val.generate(lines, cxt),
            // zero initialization used in global value allocation
            ValueKind::ZeroInit(val) => {
                let mut initialization_line = String::new();
                let ret = val.generate(&mut initialization_line, cxt);
                // we have to figure out the size of the zero-initialized `ValueData`,
                // rather than the size of a `ZeroInit`!!!
                if initialization_line.contains("<type_size>") {
                    let type_size = self.ty().size(); // If it's an array, `size()` will implement DFS and return the whole size.
                    initialization_line =
                        initialization_line.replace("<type_size>", &type_size.to_string());
                };
                append_line(lines, &initialization_line);
                ret
            }
            // initialization of a global value using an aggregate (i.e. `{ ..., {...}, ... }`)
            ValueKind::Aggregate(val) => val.generate(lines, cxt),

            //////////////////////////////////////////////////////////////////////////////////////////////
            // other instructions                                                                       //
            //////////////////////////////////////////////////////////////////////////////////////////////

            // allocation operation
            ValueKind::Alloc(val) => val.generate(lines, cxt),
            // global value allocation
            ValueKind::GlobalAlloc(val) => val.generate(lines, cxt),
            // load operation
            ValueKind::Load(val) => val.generate(lines, cxt),
            // store operation
            ValueKind::Store(val) => val.generate(lines, cxt),
            // get element pointer
            ValueKind::GetElemPtr(val) => val.generate(lines, cxt),
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

    /// Initialize a global variable using an integer.
    fn generate(&self, lines: &mut String, _cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        append_line(lines, &format!("  .word {}", self.value()));
        Ok(ValueLocation::None)
    }
}

impl RiscvGenerate for values::ZeroInit {
    type Ret = ValueLocation;

    /// Initialize a global variable using zeros.
    fn generate(&self, lines: &mut String, _cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        append_line(lines, "  .zero <type_size>");
        Ok(ValueLocation::None)
    }
}

impl RiscvGenerate for values::Aggregate {
    type Ret = ValueLocation;

    /// Initialize a global variable using an aggregate.
    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        for e in self.elems() {
            let Some(sub_value_data) = cxt.get_value_data_locally_or_globally(*e) else {
                return Err(());
            };
            sub_value_data.generate(lines, cxt)?;
        }

        Ok(ValueLocation::None)
    }
}

impl RiscvGenerate for values::Alloc {
    type Ret = ValueLocation;

    fn generate(&self, _lines: &mut String, _cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        Ok(ValueLocation::None)
    }
}

impl RiscvGenerate for values::GlobalAlloc {
    type Ret = ValueLocation;

    /// Initialize the global value by calling `generate` on the `ValueData` corresponding to the `init` field.
    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        let initializer = self.init();
        let Some(initializer_data) = cxt.get_value_data_locally_or_globally(initializer) else {
            return Err(());
        };

        initializer_data.generate(lines, cxt)
    }
}

impl RiscvGenerate for values::Load {
    type Ret = ValueLocation;

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        let src = self.src().generate(&mut String::new(), cxt)?;
        append_line(lines, &src.move_content_to_reg("t0"));
        append_line(lines, "  sw t0, <tar>");

        Ok(ValueLocation::PlaceHolder("<tar>".to_string()))
    }
}

impl RiscvGenerate for values::Store {
    type Ret = ValueLocation;

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        let val = self.value().generate(&mut String::new(), cxt)?;
        let dest = self.dest().generate(&mut String::new(), cxt)?;
        dbg!(cxt.get_value_data_locally_or_globally(self.value()).unwrap().ty());
        dbg!(cxt.get_value_data_locally_or_globally(self.dest()).unwrap().ty());

        match dest {
            ValueLocation::Stack(_) | ValueLocation::Global(_) => {
                append_line(lines, &val.move_content_to(dest.clone()));
            }
            _ => {
                return Err(());
            }
        }

        Ok(ValueLocation::None)
    }
}

impl RiscvGenerate for values::GetElemPtr {
    type Ret = ValueLocation;

    /// Generate a pointer to the element.
    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        // get two locations
        let src = self.src().generate(&mut String::new(), cxt)?;
        let idx = self.index().generate(&mut String::new(), cxt)?;

        // compute the base address to register t0
        append_line(lines, &src.move_address_to_reg("t0"));
        append_line(lines, &idx.move_content_to_reg("t1"));
        append_line(lines, "  slli t1, t1, 2");
        append_line(lines, "  add t0, t0, t1");
        append_line(lines, "  sw t0, <tar>");

        Ok(ValueLocation::PlaceHolder("<tar>".to_string()))
    }
}

impl RiscvGenerate for values::Binary {
    type Ret = ValueLocation;

    fn generate(&self, lines: &mut String, cxt: &mut ProgramContext) -> Result<Self::Ret, ()> {
        let loc_l = self.lhs().generate(&mut String::new(), cxt)?;
        let loc_r = self.rhs().generate(&mut String::new(), cxt)?;

        append_line(lines, &loc_l.move_content_to_reg("t0"));
        append_line(lines, &loc_r.move_content_to_reg("t1"));
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
        append_line(lines, &cond_loc.move_content_to_reg("t0"));

        // look up basic block names
        let Some(true_bb_data) = cxt.get_basic_block_data_in_current_function(self.true_bb()) else {
            return Err(());
        };
        let Some(false_bb_data) = cxt.get_basic_block_data_in_current_function(self.false_bb()) else {
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
        let Some(bb_data) = cxt.get_basic_block_data_in_current_function(to_bb) else {
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
            Ok(ValueLocation::None)
        } else {
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
            append_line(lines, &loc.move_content_to_reg("a0"));
        }
        append_line(lines, "<epilogue>"); // a place holder, which will be replaced by the epilogue in `FunctionData.generate`.
        append_line(lines, "  ret");

        Ok(ValueLocation::None)
    }
}
