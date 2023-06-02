use sysy_compiler::*;

fn riscv_mod_test(input: &str, output: &str) {
    run(Mode::Riscv, input, output).unwrap();
}

#[test]
fn riscv_mod_test_lv1() {
    riscv_mod_test("tests/sysy_scripts/lv1.c", "tests/riscv_scripts/lv1.asm");
}

#[test]
fn riscv_mod_test_lv2() {
    riscv_mod_test("tests/sysy_scripts/lv2.c", "tests/riscv_scripts/lv2.asm");
}

#[test]
fn riscv_mod_test_lv3_1() {
    riscv_mod_test(
        "tests/sysy_scripts/lv3-1.c",
        "tests/riscv_scripts/lv3-1.asm",
    );
}

#[test]
fn riscv_mod_test_lv3_2() {
    riscv_mod_test(
        "tests/sysy_scripts/lv3-2.c",
        "tests/riscv_scripts/lv3-2.asm",
    );
}

#[test]
fn riscv_mod_test_lv3_3() {
    riscv_mod_test(
        "tests/sysy_scripts/lv3-3.c",
        "tests/riscv_scripts/lv3-3.asm",
    );
}

#[test]
fn riscv_mod_test_lv4_1() {
    riscv_mod_test(
        "tests/sysy_scripts/lv4-1.c",
        "tests/riscv_scripts/lv4-1.asm",
    );
}