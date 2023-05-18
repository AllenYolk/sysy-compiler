use super::context::*;
use koopa::ir::entities::*;
use koopa::ir::*;

pub trait RiscvGenerate {
    fn generate(&self, cxt: &mut ProgramContext) -> Result<String, ()>;
}

impl RiscvGenerate for Program {
    fn generate(&self, cxt: &mut ProgramContext) -> Result<String, ()> {
        let mut target = String::new();

        // global values
        let mut first_time = true;
        for &val in self.inst_layout() {
            let val_data = self.borrow_value(val);
            if let Some(val_name) = val_data.name() {
                if first_time {
                    target.push_str("  .data\n");
                    first_time = false;
                }
                target.push_str(&format!("  .globl {}\n", val_name));
            }
        }

        // function names
        first_time = true;
        for &func in self.func_layout() {
            let func_name = &self.func(func).name()[1..];
            if first_time {
                target.push_str("  .text\n");
            }
            target.push_str(&format!("  .globl {}\n", func_name))
        }

        // function definitions
        for &func in self.func_layout() {
            cxt.func = Some(func);
            let func_data = self.func(func);
            target.push_str(&func_data.generate(cxt)?);
        }

        Ok(target)
    }
}

impl RiscvGenerate for FunctionData {
    fn generate(&self, cxt: &mut ProgramContext) -> Result<String, ()> {
        let func_name = &self.name()[1..];
        let mut target = format!("{}:\n", func_name);

        for (_bb, node) in self.layout().bbs() {
            for &inst_val in node.insts().keys() {
                let inst_val_data = self.dfg().value(inst_val);
                target.push_str(&inst_val_data.generate(cxt)?);
            }
        }

        Ok(target)
    }
}

impl RiscvGenerate for ValueData {
    fn generate(&self, cxt: &mut ProgramContext) -> Result<String, ()> {
        match self.kind() {
            ValueKind::Return(val) => val.generate(cxt),
            _ => Err(()),
        }
    }
}

impl RiscvGenerate for values::Return {
    fn generate(&self, cxt: &mut ProgramContext) -> Result<String, ()> {
        let mut target = String::new();

        if let Some(ret_val) = self.value() {
            let Some(ret_val_data) = cxt.get_value_data(ret_val) else {
                return Err(());
            };
            match ret_val_data.kind() {
                ValueKind::Integer(v) => {
                    target.push_str(&format!("  li a0, {}\n", v.value()));
                }
                _ => {
                    return Err(());
                }
            }
        }
        target.push_str("  ret");

        Ok(target)
    }
}
