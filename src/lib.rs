mod ast_generate;
pub mod ir_generate;
mod target_generate;
mod tools;
use std::fs;

#[derive(Debug)]
pub enum Mode {
    Koopa,
    Riscv,
    Perf,
}

#[derive(Debug)]
pub enum RunError {
    ReadFileError,
    WriteFileError,
    Sysy2AstError,
    Ast2KoopaTextError,
    KoopaText2ProgramError,
    KoopaProgram2RiscvError,
    NotImplementedError,
}

pub fn run(mode: Mode, input: &str, output: &str) -> Result<(), RunError> {
    // read the SysY input source file
    let Ok(input_content) = fs::read_to_string(input) else {
        return Err(RunError::ReadFileError);
    };

    // parse the SysY file and generate the AST
    let Ok(ast) = ast_generate::parse_sysy_to_ast(&input_content) else {
        return Err(RunError::Sysy2AstError);
    };
    println!("{}\nAST:\n", "=====".repeat(20));
    println!("{:#?}", &ast);

    // scan the AST and get the Koopa text
    let Ok(text) = ir_generate::parse_ast_to_koopa_text(&ast) else {
        return Err(RunError::Ast2KoopaTextError);
    };
    println!("{}\nKoopa:\n", "=====".repeat(20));
    println!("{}", &text);

    // write Koopa text to file
    if let Mode::Koopa = mode {
        let Ok(_) = fs::write(output, text) else {
            return Err(RunError::WriteFileError);
        };
        return Ok(());
    }

    // convert the Koopa text to Koopa program
    let Ok(program) = ir_generate::get_koopa_program(&text) else {
        return Err(RunError::KoopaText2ProgramError);
    };

    // convert the Koopa program to RISC-V text
    let Ok(rvtext) = target_generate::parse_koopa_program_to_riscv(&program) else {
        return Err(RunError::KoopaProgram2RiscvError);
    };
    println!("{}\nRISC-V:\n", "=====".repeat(20));
    println!("{}", &rvtext);

    // write RISC-V text to file
    let Ok(_) = fs::write(output, rvtext) else {
        return Err(RunError::WriteFileError);
    };

    Ok(())
}
