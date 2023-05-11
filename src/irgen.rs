mod kpgen;

use crate::astgen::ast::*;
use koopa::back::KoopaGenerator;
use koopa::ir::builder_traits::*;
use koopa::ir::*;
use kpgen::KoopaGenerate;

#[allow(dead_code)]
const CORRECT_PROGRAM_TEXT: &str = r#"fun @main(): i32 {
%entry:
  ret 0
}
"#;

/// Convert the AST to the Koopa program.
pub fn parse_ast(ast: &CompUnit) -> Result<Program, ()> {
    // create an empty Koopa program
    let mut program = Program::new();

    // scan the AST recursively, and fill things into the Koopa program
    ast.generate(&mut program)?;
    Ok(program)
}

/// Convert a Koopa program to its text form.
pub fn get_program_text(program: &Program) -> Result<String, ()> {
    let mut gen = KoopaGenerator::new(Vec::new());
    gen.generate_on(program).map_err(|_| ())?;

    match std::str::from_utf8(&gen.writer()) {
        Ok(text) => Ok(String::from(text)),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use koopa::ir::builder_traits::{BasicBlockBuilder, ValueBuilder};

    use super::*;

    #[test]
    fn program2text_test() {
        let mut program = Program::new();
        let main_f = program.new_func(FunctionData::with_param_names(
            "@main".into(),
            Vec::new(),
            Type::get_i32(),
        ));
        let main_fd = program.func_mut(main_f);
        let entry_bb = main_fd
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%entry".into()));
        main_fd.layout_mut().bbs_mut().extend([entry_bb]);

        let zero_v = main_fd.dfg_mut().new_value().integer(0);
        let ret_v = main_fd.dfg_mut().new_value().ret(Some(zero_v));
        main_fd
            .layout_mut()
            .bb_mut(entry_bb)
            .insts_mut()
            .push_key_back(ret_v)
            .unwrap();

        assert_eq!(CORRECT_PROGRAM_TEXT, get_program_text(&program).unwrap());
    }
}
