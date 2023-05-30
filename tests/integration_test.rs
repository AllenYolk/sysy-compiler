use std::fs;
use sysy_compiler::*;

const CORRECT_RISCV_TEXT: &str = r#"  .text
  .globl main
main:
  li a0, 112
  ret"#;

#[test]
fn riscv_mod_test_lv1() {
    run(
        Mode::Riscv,
        "tests/sysy_scripts/lv1.c",
        "tests/riscv_scripts/lv1.asm",
    )
    .unwrap();
    let res = fs::read_to_string("tests/riscv_scripts/lv1.asm").unwrap();
    assert_eq!(res, CORRECT_RISCV_TEXT);
}
