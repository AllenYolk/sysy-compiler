# sysy-compiler

## Introduction

A SysY compiler for PKU's Compiler Principle course, implemented in Rust.

* SysY source code -> AST: [lalrpop](https://crates.io/crates/lalrpop)
* AST -> Koopa Text (an intermediate representation)
* Koopa Text -> Koopa Program: [koopa](https://crates.io/crates/koopa)
* Koopa Program -> RISC-V

For more information, see the [report](./lab-report.md) (written in Chinese 🫡).

## Results

* [Offline Judgement](https://github.com/pku-minic/compiler-dev-test-cases):
  * all the tests (except performance tests) have been passed!
* Online Judgement:
  * SysY to Koopa: **99.09** / 100
  * SysY to RISC-V: **98.64** / 100
  * Performance: **100** / 100 (598.74 sec 😋👉🤡)

## Notices

* **Disclaimer**: for your own benefit, please avoid directly copying and pasting the code 🙏.
* If you find my code or idea helpful, please consider citing this repository in your lab report or code 😘.
  * ...... and star ⭐️ the repo if you like!

## References

* [Kira (Rust version)](https://github.com/pku-minic/kira-rs)
* [PKU Compiler Online Document](https://pku-minic.github.io/online-doc/#/)
