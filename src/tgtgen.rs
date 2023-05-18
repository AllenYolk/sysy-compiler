mod context;
mod rvgen;

use context::ProgramContext;
use koopa::ir::*;
use rvgen::RiscvGenerate;

#[allow(dead_code)]
const CORRECT_RISCV_TEXT: &str = r#"  .text
  .globl main
main:
  li a0, 0
  ret"#;

/// Convert the Koopa program to RISC-V text.
pub fn parse_koopa_program_to_riscv(program: &Program) -> Result<String, ()> {
    program.generate(&mut ProgramContext::new(program))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::irgen::{get_koopa_program, CORRECT_KOOPA_TEXT};

    #[test]
    fn riscv_generate_test() {
        let rv = parse_koopa_program_to_riscv(&get_koopa_program(CORRECT_KOOPA_TEXT).unwrap());
        assert_eq!(rv.unwrap(), CORRECT_RISCV_TEXT);
    }
}
