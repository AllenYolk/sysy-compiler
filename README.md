# sysy-compiler

A SysY compiler for PKU's Compiler Principle course, implemented in Rust.

* SysY source code -> AST: [lalrpop](https://crates.io/crates/lalrpop)
* AST -> Koopa Text (an intermediate representation)
* Koopa Text -> Koopa Program: [koopa](https://crates.io/crates/koopa)
* Koopa Program -> RISC-V