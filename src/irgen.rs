mod kpgen;
mod scopes;
mod tempvar;

use crate::astgen::ast::*;
use koopa::front::Driver;
use koopa::ir::*;
use kpgen::KoopaTextGenerate;
use scopes::Scopes;
use tempvar::TempVariableManager;

/// Convert the AST to the Koopa text.
pub fn parse_ast_to_koopa_text(ast: &CompUnit) -> Result<String, ()> {
    let mut scopes = Scopes::new();
    let mut tvm = TempVariableManager::new();
    ast.generate(&mut scopes, &mut tvm)
}

/// Convert a Koopa text to Koopa program.
pub fn get_koopa_program(text: &str) -> Result<Program, ()> {
    let driver = Driver::from(text);

    driver.generate_program().map_err(|_| ())
}
