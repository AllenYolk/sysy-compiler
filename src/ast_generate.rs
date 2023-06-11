pub mod ast;
use ast::*;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(sysy);

/// Convert the input SysY source code to AST.
///
/// `lalrpop` crate is used to generate the parser.
/// If an error occurs, `Err(())` is returned.
/// Otherwise, return the root of the AST (i.e. `CompUnit`) wrapped by `Ok`.
///
/// # Errors
/// An error may occur when the input SysY source code is not valid.
///
/// # Examples
/// ```
/// use sysy_compiler::ast_generate::parse_sysy_to_ast;
/// let input = r#"
/// int main() {
///    return 0;
/// }
/// "#;
/// let ast = parse_sysy_to_ast(input).unwrap();
/// ```
pub fn parse_sysy_to_ast(input: &str) -> Result<CompUnit, ()> {
    sysy::CompUnitParser::new().parse(input).map_err(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn astgen_test(input: &str) {
        let input_content = std::fs::read_to_string(input).unwrap();
        let ast = parse_sysy_to_ast(&input_content).unwrap();
        dbg!(ast);
    }

    #[test]
    fn astgen_lv1_test() {
        astgen_test("tests/sysy_scripts/lv1.c");
    }

    #[test]
    fn astgen_lv2_test() {
        astgen_test("tests/sysy_scripts/lv2.c");
    }

    #[test]
    fn astgen_lv3_1_test() {
        astgen_test("tests/sysy_scripts/lv3-1.c");
    }

    #[test]
    fn astgen_lv3_2_test() {
        astgen_test("tests/sysy_scripts/lv3-2.c");
    }

    #[test]
    fn astgen_lv3_3_test() {
        astgen_test("tests/sysy_scripts/lv3-3.c");
    }

    #[test]
    fn astgen_lv4_1_test() {
        astgen_test("tests/sysy_scripts/lv4-1.c");
    }

    #[test]
    fn astgen_lv4_2_test() {
        astgen_test("tests/sysy_scripts/lv4-2.c");
    }

    #[test]
    fn astgen_lv5_test() {
        astgen_test("tests/sysy_scripts/lv5.c");
    }

    #[test]
    fn astgen_lv6_1_test() {
        astgen_test("tests/sysy_scripts/lv6-1.c");
    }

    #[test]
    fn astgen_lv6_2_test() {
        astgen_test("tests/sysy_scripts/lv6-2.c");
    }

    #[test]
    fn astgen_lv7_1_test() {
        astgen_test("tests/sysy_scripts/lv7-1.c");
    }

    #[test]
    fn astgen_lv7_2_test() {
        astgen_test("tests/sysy_scripts/lv7-2.c");
    }

    #[test]
    fn astgen_lv8_0_test() {
        astgen_test("tests/sysy_scripts/lv8-0.c");
    }

    #[test]
    fn astgen_lv8_1_test() {
        astgen_test("tests/sysy_scripts/lv8-1.c");
    }

    #[test]
    fn astgen_lv8_2_test() {
        astgen_test("tests/sysy_scripts/lv8-2.c");
    }

    #[test]
    fn astgen_lv8_3_test() {
        astgen_test("tests/sysy_scripts/lv8-3.c");
    }

    #[test]
    fn astgen_lv9_1_test() {
        astgen_test("tests/sysy_scripts/lv9-1.c");
    }

    #[test]
    fn astgen_lv9_2_test() {
        astgen_test("tests/sysy_scripts/lv9-2.c");
    }

    #[test]
    fn astgen_lv9_3_test() {
        astgen_test("tests/sysy_scripts/lv9-3.c");
    }
}
