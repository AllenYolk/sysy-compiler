use super::scopes::Scopes;
use super::tempvar::TempVariableManager;
use crate::astgen::ast::*;

pub trait KoopaTextGenerate {
    fn generate(&self, scopes: &mut Scopes, tvm: &mut TempVariableManager) -> Result<String, ()>;
}

impl KoopaTextGenerate for CompUnit {
    fn generate(&self, scopes: &mut Scopes, tvm: &mut TempVariableManager) -> Result<String, ()> {
        self.func_def.generate(scopes, tvm)
    }
}

impl KoopaTextGenerate for FuncDef {
    fn generate(&self, scopes: &mut Scopes, tvm: &mut TempVariableManager) -> Result<String, ()> {
        let text = format!(
            "fun @{}(): {} {{\n{}\n}}",
            self.ident,
            self.func_type.generate(scopes, tvm)?,
            self.block.generate(scopes, tvm)?
        );
        Ok(text)
    }
}

impl KoopaTextGenerate for FuncType {
    fn generate(&self, scopes: &mut Scopes, tvm: &mut TempVariableManager) -> Result<String, ()> {
        match self {
            Self::Int => Ok(String::from("i32")),
            // _ => Err(()),
        }
    }
}

impl KoopaTextGenerate for Block {
    fn generate(&self, scopes: &mut Scopes, tvm: &mut TempVariableManager) -> Result<String, ()> {
        let text = format!("%entry:\n{}", self.stmt.generate(scopes, tvm)?);
        Ok(text)
    }
}

impl KoopaTextGenerate for Stmt {
    fn generate(&self, scopes: &mut Scopes, tvm: &mut TempVariableManager) -> Result<String, ()> {
        let text = format!("  ret {}", 1);
        Ok(text)
    }
}
