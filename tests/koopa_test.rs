use std::fs;
use sysy_compiler::*;

fn koopa_mod_test(input: &str, output: &str) {
    run(Mode::Koopa, input, output).unwrap();
    let text = fs::read_to_string(output).unwrap();
    ir_generate::get_koopa_program(&text).unwrap();
}

#[test]
fn koopa_mod_test_lv1() {
    koopa_mod_test("tests/sysy_scripts/lv1.c", "tests/koopa_scripts/lv1.koopa");
}

#[test]
fn koopa_mod_test_lv2() {
    koopa_mod_test("tests/sysy_scripts/lv2.c", "tests/koopa_scripts/lv2.koopa");
}

#[test]
fn koopa_mod_test_lv3_1() {
    koopa_mod_test(
        "tests/sysy_scripts/lv3-1.c",
        "tests/koopa_scripts/lv3-1.koopa",
    );
}

#[test]
fn koopa_mod_test_lv3_2() {
    koopa_mod_test(
        "tests/sysy_scripts/lv3-2.c",
        "tests/koopa_scripts/lv3-2.koopa",
    );
}

#[test]
fn koopa_mod_test_lv3_3() {
    koopa_mod_test(
        "tests/sysy_scripts/lv3-3.c",
        "tests/koopa_scripts/lv3-3.koopa",
    );
}

#[test]
fn koopa_mod_test_lv4_1() {
    koopa_mod_test(
        "tests/sysy_scripts/lv4-1.c",
        "tests/koopa_scripts/lv4-1.koopa",
    );
}
