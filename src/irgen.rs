mod kpgen;
mod scopes;
mod tempvar;

use crate::astgen::ast::*;
use koopa::front::Driver;
use koopa::ir::*;
use kpgen::KoopaTextGenerate;
use scopes::Scopes;
use tempvar::TempSymbolManager;

/// Convert the AST to the Koopa text.
pub fn parse_ast_to_koopa_text(ast: &CompUnit) -> Result<String, ()> {
    let mut text = String::new();
    ast.generate(&mut text, &mut Scopes::new(), &mut TempSymbolManager::new())?;
    Ok(text)
}

/// Convert a Koopa text to Koopa program.
pub fn get_koopa_program(text: &str) -> Result<Program, ()> {
    let driver = Driver::from(text);

    driver.generate_program().map_err(|_| ())
}
