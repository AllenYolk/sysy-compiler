mod kpgen;

use crate::astgen::ast::*;
use koopa::front::Driver;
use koopa::ir::*;
use kpgen::KoopaTextGenerate;

/// Convert the AST to the Koopa text.
pub fn parse_ast_to_koopa_text(ast: &CompUnit) -> Result<String, ()> {
    ast.generate()
}

/// Convert a Koopa text to Koopa program.
pub fn get_koopa_program(text: &str) -> Result<Program, ()> {
    let driver = Driver::from(text);

    driver.generate_program().map_err(|_| ())
}
