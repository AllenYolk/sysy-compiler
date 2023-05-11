use crate::astgen::ast::*;
use koopa::ir::*;
use koopa::ir::builder_traits::*;

pub trait KoopaGenerate {
    type Return;

    fn generate(&self, program: &mut Program) -> Result<Self::Return, ()>;
}

impl KoopaGenerate for CompUnit {
    type Return = ();

    fn generate(&self, program: &mut Program) -> Result<Self::Return, ()> {
        self.func_def.generate(program)
    }
}

impl KoopaGenerate for FuncDef {
    type Return = ();

    fn generate(&self, program: &mut Program) -> Result<Self::Return, ()> {
        let mut func_data = FunctionData::with_param_names(
            format!("@{}", self.ident), 
            Vec::new(), 
            self.func_type.generate(program)?,
        );
        let return_value = self.block.generate(program)?;
        let func = program.new_func(func_data);
        let func_data = program.func_mut(func);

        let entry_bb = func_data
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%entry".into()));
        func_data.layout_mut().bbs_mut().extend([entry_bb]);

        let zero_v = func_data.dfg_mut().new_value().integer(return_value);
        let ret_v = func_data.dfg_mut().new_value().ret(Some(zero_v));
        func_data
            .layout_mut()
            .bb_mut(entry_bb)
            .insts_mut()
            .extend([ret_v]);

        
        Ok(())
    }
}

impl KoopaGenerate for FuncType {
    type Return = Type;

    fn generate(&self, program: &mut Program) -> Result<Self::Return, ()> {
        match self {
            Self::Int => Ok(Type::get_i32()),
            // _ => Err(()), 
        }
    }
}

impl KoopaGenerate for Block {
    type Return = i32;

    fn generate(&self, program: &mut Program) -> Result<Self::Return, ()> {
        self.stmt.generate(program)
    }
}

impl KoopaGenerate for Stmt {
    type Return = i32;

    fn generate(&self, program: &mut Program) -> Result<Self::Return, ()> {
        Ok(self.num)
    }
}
