pub mod astgen;
pub mod irgen;
use std::fs;

#[derive(Debug)]
pub enum Mode {
    Koopa,
    Riscv,
    Perf,
}

pub enum RunError {
    ReadFileError,
    WriteFileError,
    Sysy2AstError,
    Ast2KoopaTextError,
    KoopaText2ProgramError,
    NotImplementedError,
}

pub fn run(mode: Mode, input: &str, output: &str) -> Result<(), RunError> {
    // read the SysY input source file
    let Ok(input_content) = fs::read_to_string(input) else {
        return Err(RunError::ReadFileError);
    };

    // parse the SysY file and generate the AST
    let Ok(ast) = astgen::parse_sysy_to_ast(&input_content) else {
        return Err(RunError::Sysy2AstError);
    };
    dbg!(&ast);

    // scan the AST and get the Koopa text
    let Ok(text) = irgen::parse_ast_to_text(&ast) else {
        return Err(RunError::Ast2KoopaTextError);
    };

    if let Mode::Koopa = mode {
        let Ok(_) = fs::write(output, text) else {
            return Err(RunError::WriteFileError);
        };
        return Ok(());
    } 

    // convert the Koopa text to Koopa program
    let Ok(_program) = irgen::get_program(&text) else {
        return Err(RunError::KoopaText2ProgramError);
    };

    return Err(RunError::NotImplementedError); 
}
