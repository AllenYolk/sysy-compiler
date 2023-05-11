use std::env;
use std::process::exit;
use std::fs;
use sysy_compiler::astgen;

#[derive(Debug)]
enum Mode {
    Koopa,
    Riscv,
    Perf,
}

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
                    output: output_
                })
            },
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
        eprintln!("Invalid Command Line Argument!\n{}", CLI_HELP);
        exit(-1)
    };
    println!("{:?}, {}, {}", mode, input, output);

    // read the input source file
    let Ok(input_content) = fs::read_to_string(&input) else {
        eprintln!("Invalid SysY Input File: {}", input);
        exit(-1);
    };

    let Ok(ast) = astgen::parse_sysy(&input_content) else {
        eprintln!("Parsing Error!");
        exit(-1);
    };
    dbg!(ast);
}
