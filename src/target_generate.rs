mod context;
mod riscv_generate;

use context::ProgramContext;
use koopa::ir::*;
use riscv_generate::RiscvGenerate;

/// Convert the Koopa program to RISC-V text.
pub fn parse_koopa_program_to_riscv(program: &Program) -> Result<String, ()> {
    program.generate(&mut ProgramContext::new(program))
}
