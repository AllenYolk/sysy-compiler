use crate::astgen::ast::*;
use super::scopes::Scopes;

pub trait KoopaTextGenerate {
    fn generate(&self, scopes: &mut Scopes) -> Result<String, ()>;
}

impl KoopaTextGenerate for CompUnit {
    fn generate(&self, scopes: &mut Scopes) -> Result<String, ()> {
        self.func_def.generate(scopes)
    }
}

impl KoopaTextGenerate for FuncDef {
    fn generate(&self, scopes: &mut Scopes) -> Result<String, ()> {
        let text = format!(
            "fun @{}(): {} {{\n{}\n}}",
            self.ident,
            self.func_type.generate(scopes)?,
            self.block.generate(scopes)?
        );
        Ok(text)
    }
}

impl KoopaTextGenerate for FuncType {
    fn generate(&self, scopes: &mut Scopes) -> Result<String, ()> {
        match self {
            Self::Int => Ok(String::from("i32")),
            // _ => Err(()),
        }
    }
}

impl KoopaTextGenerate for Block {
    fn generate(&self, scopes: &mut Scopes) -> Result<String, ()> {
        let text = format!("%entry:\n{}", self.stmt.generate(scopes)?);
        Ok(text)
    }
}

impl KoopaTextGenerate for Stmt {
    fn generate(&self, scopes: &mut Scopes) -> Result<String, ()> {
        let text = format!("  ret {}", 1);
        Ok(text)
    }
}
