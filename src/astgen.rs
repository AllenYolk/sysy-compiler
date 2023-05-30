pub mod ast;
use ast::*;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(sysy);

/// Convert the input SysY source code to the AST.
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
    fn astgen_lv3_test() {
        astgen_test("tests/sysy_scripts/lv3.c");
    }
}
