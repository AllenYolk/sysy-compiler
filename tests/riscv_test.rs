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

#[test]
fn riscv_mod_test_lv4_2() {
    riscv_mod_test(
        "tests/sysy_scripts/lv4-2.c",
        "tests/riscv_scripts/lv4-2.asm",
    );
}

#[test]
fn riscv_mod_test_lv5() {
    riscv_mod_test("tests/sysy_scripts/lv5.c", "tests/riscv_scripts/lv5.asm");
}

#[test]
fn riscv_mod_test_lv6_1() {
    riscv_mod_test(
        "tests/sysy_scripts/lv6-1.c",
        "tests/riscv_scripts/lv6-1.asm",
    );
}

#[test]
fn riscv_mod_test_lv6_2() {
    riscv_mod_test(
        "tests/sysy_scripts/lv6-2.c",
        "tests/riscv_scripts/lv6-2.asm",
    );
}

#[test]
fn riscv_mod_test_lv7_1() {
    riscv_mod_test(
        "tests/sysy_scripts/lv7-1.c",
        "tests/riscv_scripts/lv7-1.asm",
    );
}

#[test]
fn riscv_mod_test_lv7_2() {
    riscv_mod_test(
        "tests/sysy_scripts/lv7-2.c",
        "tests/riscv_scripts/lv7-2.asm",
    )
}

#[test]
fn riscv_mod_test_lv8_0() {
    riscv_mod_test(
        "tests/sysy_scripts/lv8-0.c",
        "tests/riscv_scripts/lv8-0.asm",
    );
}

#[test]
fn riscv_mod_test_lv8_1() {
    riscv_mod_test(
        "tests/sysy_scripts/lv8-1.c",
        "tests/riscv_scripts/lv8-1.asm",
    );
}

#[test]
fn riscv_mod_test_lv8_2() {
    riscv_mod_test(
        "tests/sysy_scripts/lv8-2.c",
        "tests/riscv_scripts/lv8-2.asm",
    );
}
