mod kpgen;

use crate::astgen::ast::*;
use koopa::front::Driver;
use koopa::ir::*;
use kpgen::KoopaTextGenerate;

#[allow(dead_code)]
const CORRECT_PROGRAM_TEXT: &str = r#"fun @main(): i32 {
%entry:
  ret 0
}
"#;

/// Convert the AST to the Koopa text.
pub fn parse_ast_to_text(ast: &CompUnit) -> Result<String, ()> {
    // scan the AST recursively, and fill things into the Koopa program
    // ast.generate(&mut program)?;
    ast.generate()
}

/// Convert a Koopa text to Koopa program.
pub fn get_program(text: &str) -> Result<Program, ()> {
    let driver: Driver<_> = text.into();

    driver.generate_program().map_err(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text2program_test() {
        get_program(CORRECT_PROGRAM_TEXT).unwrap();
    }
}
