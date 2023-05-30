mod context;
mod rvgen;

use context::ProgramContext;
use koopa::ir::*;
use rvgen::RiscvGenerate;

/// Convert the Koopa program to RISC-V text.
pub fn parse_koopa_program_to_riscv(program: &Program) -> Result<String, ()> {
    program.generate(&mut ProgramContext::new(program))
}
