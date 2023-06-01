mod koopa_generate;
mod scopes;
mod temp_symbol;

use crate::ast_generate::ast::*;
use koopa::front::Driver;
use koopa::ir::*;
use koopa_generate::KoopaTextGenerate;
use scopes::Scopes;
use temp_symbol::TempSymbolManager;

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