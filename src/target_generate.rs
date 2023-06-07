mod context;
mod riscv_generate;
mod value_location;
mod function_scan;

use context::ProgramContext;
use koopa::ir::*;
use riscv_generate::RiscvGenerate;

/// Convert the Koopa program to RISC-V text.
///
/// The only argument is a reference to the Koopa program (i.e. `&Program`), which is defined in the `koopa` crate.
/// If an error occurs, `Err(())` is returned.
/// Otherwise, return the RISC-V text wrapped by `Ok`.
///
/// # Errors
/// An error may occur when the Koopa program is not valid.
pub fn parse_koopa_program_to_riscv(program: &Program) -> Result<String, ()> {
    let mut text = String::new();
    program.generate(&mut text, &mut ProgramContext::new(program))?;
    Ok(text)
}
