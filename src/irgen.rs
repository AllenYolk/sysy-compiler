mod kpgen;

use crate::astgen::ast::*;
use koopa::front::Driver;
use koopa::ir::*;
use kpgen::KoopaTextGenerate;

#[allow(dead_code)]
pub const CORRECT_KOOPA_TEXT: &str = r#"fun @main(): i32 {
%entry:
  ret 0
}
"#;

/// Convert the AST to the Koopa text.
pub fn parse_ast_to_koopa_text(ast: &CompUnit) -> Result<String, ()> {
    ast.generate()
}

/// Convert a Koopa text to Koopa program.
pub fn get_koopa_program(text: &str) -> Result<Program, ()> {
    let driver = Driver::from(text);

    driver.generate_program().map_err(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text2program_test() {
        get_koopa_program(CORRECT_KOOPA_TEXT).unwrap();
    }
}
