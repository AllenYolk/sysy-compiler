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

#[test]
fn koopa_mod_test_lv4_2() {
    koopa_mod_test(
        "tests/sysy_scripts/lv4-2.c",
        "tests/koopa_scripts/lv4-2.koopa",
    );
}

#[test]
fn koopa_mod_test_lv5() {
    koopa_mod_test("tests/sysy_scripts/lv5.c", "tests/koopa_scripts/lv5.koopa");
}

#[test]
fn koopa_mod_test_lv6_1() {
    koopa_mod_test(
        "tests/sysy_scripts/lv6-1.c",
        "tests/koopa_scripts/lv6-1.koopa",
    );
}

#[test]
fn koopa_mod_test_lv6_2() {
    koopa_mod_test(
        "tests/sysy_scripts/lv6-2.c",
        "tests/koopa_scripts/lv6-2.koopa",
    );
}

#[test]
fn koopa_mod_test_lv7_1() {
    koopa_mod_test(
        "tests/sysy_scripts/lv7-1.c",
        "tests/koopa_scripts/lv7-1.koopa",
    );
}

#[test]
fn koopa_mod_test_lv7_2() {
    koopa_mod_test(
        "tests/sysy_scripts/lv7-2.c",
        "tests/koopa_scripts/lv7-2.koopa",
    )
}

#[test]
fn koopa_mod_test_lv8_0() {
    koopa_mod_test(
        "tests/sysy_scripts/lv8-0.c",
        "tests/koopa_scripts/lv8-0.koopa",
    );
}

#[test]
fn koopa_mod_test_lv8_1() {
    koopa_mod_test(
        "tests/sysy_scripts/lv8-1.c",
        "tests/koopa_scripts/lv8-1.koopa",
    );
}

#[test]
fn koopa_mod_test_lv8_2() {
    koopa_mod_test(
        "tests/sysy_scripts/lv8-2.c",
        "tests/koopa_scripts/lv8-2.koopa",
    );
}

#[test]
fn koopa_mod_test_lv8_3() {
    koopa_mod_test(
        "tests/sysy_scripts/lv8-3.c",
        "tests/koopa_scripts/lv8-3.koopa",
    );
}

#[test]
fn koopa_mod_test_lv9_1() {
    koopa_mod_test(
        "tests/sysy_scripts/lv9-1.c",
        "tests/koopa_scripts/lv9-1.koopa",
    );
}

#[test]
fn koopa_mod_test_lv9_2() {
    koopa_mod_test(
        "tests/sysy_scripts/lv9-2.c",
        "tests/koopa_scripts/lv9-2.koopa",
    );
}

#[test]
fn koopa_mod_test_lv9_3() {
    koopa_mod_test(
        "tests/sysy_scripts/lv9-3.c",
        "tests/koopa_scripts/lv9-3.koopa",
    );
}