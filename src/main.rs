use std::env;
use std::process::exit;
use sysy_compiler::{Mode, RunError};

struct Cli {
    mode: Mode,
    input: String,
    output: String,
}

impl Cli {
    fn parse() -> Result<Self, ()> {
        let mut args = env::args();
        args.next();
        let mode = args.next();
        let input = args.next();
        let o = args.next();
        let output = args.next();

        match (mode, input, o, output) {
            (Some(mode_), Some(input_), Some(o_), Some(output_)) if o_ == "-o" => {
                let mode_converted = match mode_.as_str() {
                    "-koopa" => Mode::Koopa,
                    "-riscv" => Mode::Riscv,
                    "-perf" => Mode::Perf,
                    _ => return Err(()),
                };
                Ok(Self {
                    mode: mode_converted,
                    input: input_,
                    output: output_,
                })
            }
            _ => Err(()),
        }
    }
}

const CLI_HELP: &str = r#"
sysy compiler: Yifan Huang <1900012913@pku.edu.cn>

Usage 1: cargo run -- MODE INPUT -o OUTPUT
Usage 2: <path-to-sysy_compiler> MODE INPUT -o OUTPUT
    MODE: "-koopa", "-riscv" or "-perf"
    INPUT: the input SysY source file
    OUTPUT: the output file
"#;

fn main() {
    // parse the command line arguments
    let Ok(Cli{mode, input, output}) = Cli::parse() else {
        eprintln!("Error: invalid command line argument!\n{}", CLI_HELP);
        exit(-1)
    };
    println!("mode={:?}, input={}, output={}", mode, input, output);

    match sysy_compiler::run(mode, &input, &output) {
        Err(e) => {
            match e {
                RunError::ReadFileError => {
                    eprintln!("Error: cannot read input file {}!", &input);
                }
                RunError::WriteFileError => {
                    eprintln!("Error: cannot write file {}!", &output)
                }
                RunError::Sysy2AstError => {
                    eprintln!("Error: cannot convert SysY to AST!");
                }
                RunError::Ast2KoopaTextError => {
                    eprintln!("Error: cannot convert AST to Koopa text!");
                }
                RunError::KoopaText2ProgramError => {
                    eprintln!("Error: cannot convert Koopa text to program!");
                }
                RunError::KoopaProgram2RiscvError => {
                    eprintln!("Error: cannot convert Koopa program to RISC-V");
                }
                RunError::NotImplementedError => {
                    eprintln!("Error: not implemented");
                }
            }
            exit(-1);
        }
        Ok(_) => (),
    }
}
