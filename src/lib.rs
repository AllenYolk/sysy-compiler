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
    Ast2KoopaError,
    Koopa2TextError,
    NotImplementedError,
}

pub fn run(mode: Mode, input: &str, output: &str) -> Result<(), RunError> {
    // read the SysY input source file
    let Ok(input_content) = fs::read_to_string(input) else {
        return Err(RunError::ReadFileError);
    };

    // parse the SysY file and generate the AST
    let Ok(ast) = astgen::parse_sysy(&input_content) else {
        return Err(RunError::Sysy2AstError);
    };
    dbg!(&ast);

    // scan the AST and get the Koopa program
    let Ok(program) = irgen::parse_ast(&ast) else {
        return Err(RunError::Ast2KoopaError);
    };

    // convert the Koopa program to text form
    let Ok(text) = irgen::get_program_text(&program) else {
        return Err(RunError::Koopa2TextError);
    };

    match mode {
        Mode::Koopa => {
            let Ok(_) = fs::write(output, text) else {
                return Err(RunError::WriteFileError);
            };
        },
        _ => { 
            return Err(RunError::NotImplementedError); 
        }
    }
    

    Ok(())
}
