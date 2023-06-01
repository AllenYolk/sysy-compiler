mod context;
mod riscv_generate;
mod value_location;

use context::ProgramContext;
use koopa::ir::*;
use riscv_generate::RiscvGenerate;

/// Convert the Koopa program to RISC-V text.
pub fn parse_koopa_program_to_riscv(program: &Program) -> Result<String, ()> {
    let mut text = String::new();
    program.generate(&mut text, &mut ProgramContext::new(program))?;
    Ok(text)
}
