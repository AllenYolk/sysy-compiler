use crate::astgen::ast::*;

pub trait KoopaTextGenerate {
    fn generate(&self) -> Result<String, ()>;
}

impl KoopaTextGenerate for CompUnit {
    fn generate(&self) -> Result<String, ()> {
        self.func_def.generate()
    }
}

impl KoopaTextGenerate for FuncDef {
    fn generate(&self) -> Result<String, ()> {
        let text = format!(
            "fun @{}(): {} {{\n{}\n}}",
            self.ident,
            self.func_type.generate()?,
            self.block.generate()?
        );
        Ok(text)
    }
}

impl KoopaTextGenerate for FuncType {
    fn generate(&self) -> Result<String, ()> {
        match self {
            Self::Int => Ok(String::from("i32")),
            // _ => Err(()),
        }
    }
}

impl KoopaTextGenerate for Block {
    fn generate(&self) -> Result<String, ()> {
        let text = format!("%entry:\n{}", self.stmt.generate()?);
        Ok(text)
    }
}

impl KoopaTextGenerate for Stmt {
    fn generate(&self) -> Result<String, ()> {
        let text = format!("  ret {}", self.num);
        Ok(text)
    }
}
