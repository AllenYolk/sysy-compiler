use super::function_call::function_arg_location;

/// The location of a value.
///
/// The `String` contained in the variants can be used directly in RISC-V instructions!
#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum ValueLocation {
    /// An Immediate value.
    Imm(String),
    /// Located in a register.
    Reg(String),
    /// Located on the stack frame.
    Stack(String),
    /// Global value
    Global(String),
    /// A placeholder.
    ///
    /// Used in the implementation of `FunctionData.generate(...)`,
    /// in order to put the value location (returned by `cxt.get_value_location(Value)`) at a correct place.
    PlaceHolder(String),
    None,
}

impl ValueLocation {
    /// Generate the instruction (a `String`) that moves the value (with the location) to the given register.
    pub fn move_content_to_reg(&self, reg: &str) -> String {
        match self {
            Self::Imm(val) => {
                format!("  li {}, {}", reg, val)
            }
            Self::Reg(r) => {
                format!("  mv {}, {}", reg, r)
            }
            Self::Stack(addr) => {
                format!("  lw {}, {}", reg, addr)
            }
            Self::Global(s) => {
                format!("  la t0, {}\n  lw {}, 0(t0)", s, reg)
            }
            _ => String::new(),
        }
    }

    /// Generate the instruction (a `String`) that moves the value (with the location) to the given stack address.
    pub fn move_content_to_stack(&self, addr: &str) -> String {
        match self {
            Self::Imm(val) => {
                format!("  li t0, {}\n  sw t0, {}", val, addr)
            }
            Self::Reg(r) => {
                format!("  sw {}, {}", r, addr)
            }
            Self::Stack(addr2) => {
                format!("  lw t0, {}\n  sw t0, {}", addr2, addr)
            }
            Self::Global(s) => {
                format!("  la t0, {}\n  lw t0, 0(t0)\n  sw t0, {}", s, addr)
            }
            _ => String::new(),
        }
    }

    pub fn move_content_to_global(&self, name: &str) -> String {
        match self {
            Self::Imm(val) => {
                format!("  li t0, {}\n  la t1, {}\n  sw t0, 0(t1)", val, name)
            }
            Self::Reg(r) => {
                format!("  la t0, {}\n  sw {}, 0(t0)", name, r)
            }
            Self::Stack(addr) => {
                format!("  la t0, {}\n  lw t1, {}\n  sw t1, 0(t0)", name, addr)
            }
            Self::Global(s) => {
                format!(
                    "  la t0, {}\n  lw t0, 0(t0)\n  la t1, {}\n  sw t0, 0(t1)",
                    s, name
                )
            }
            _ => String::new(),
        }
    }

    /// Generate the instruction (a `String`) that moves the value (with the location) to the given destination.
    pub fn move_content_to(&self, dest: ValueLocation) -> String {
        match dest {
            Self::Reg(r) => self.move_content_to_reg(&r),
            Self::Stack(addr) => self.move_content_to_stack(&addr),
            Self::Global(name) => self.move_content_to_global(&name),
            _ => String::new(),
        }
    }

    pub fn move_address_to_reg(&self, reg: &str) -> String {
        match self {
            Self::Stack(addr) => {
                // `addr` has the form `offset(base)`, where `offset` is a number and `base` is a register.
                // We need to extract `offset` and `base` from `addr`.
                let splitted_result: Vec<&str> = addr.split(|c| c == '(' || c == ')').collect();
                format!(
                    "  addi {}, {}, {}",
                    reg, splitted_result[1], splitted_result[0]
                )
            }
            Self::Global(s) => {
                format!("  la {}, {}", reg, s)
            }
            _ => String::new(),
        }
    }

    /// Generate the instruction (a `String`) that treat the value as the i-th argument for a function being called now.
    pub fn act_as_function_arg(&self, i: usize) -> String {
        let dest = function_arg_location(i, 0, true);
        self.move_content_to(dest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_content_to_test() {
        assert_eq!(
            ValueLocation::Imm("1".into()).move_content_to(ValueLocation::Reg("a0".into())),
            "  li a0, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).move_content_to(ValueLocation::Stack("0(sp)".into())),
            "  li t0, 1\n  sw t0, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).move_content_to(ValueLocation::Global("a".into())),
            "  li t0, 1\n  la t1, a\n  sw t0, 0(t1)"
        );

        assert_eq!(
            ValueLocation::Reg("a0".into()).move_content_to(ValueLocation::Reg("a1".into())),
            "  mv a1, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).move_content_to(ValueLocation::Stack("0(sp)".into(),)),
            "  sw a0, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).move_content_to(ValueLocation::Global("a".into())),
            "  la t0, a\n  sw a0, 0(t0)"
        );

        assert_eq!(
            ValueLocation::Stack("0(sp)".into(),).move_content_to(ValueLocation::Reg("a0".into())),
            "  lw a0, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into(),)
                .move_content_to(ValueLocation::Stack("4(sp)".into(),)),
            "  lw t0, 0(sp)\n  sw t0, 4(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into(),)
                .move_content_to(ValueLocation::Global("a".into())),
            "  la t0, a\n  lw t1, 0(sp)\n  sw t1, 0(t0)"
        );

        assert_eq!(
            ValueLocation::Global("a".into()).move_content_to(ValueLocation::Reg("a0".into())),
            "  la t0, a\n  lw a0, 0(t0)"
        );
        assert_eq!(
            ValueLocation::Global("a".into())
                .move_content_to(ValueLocation::Stack("0(sp)".into(),)),
            "  la t0, a\n  lw t0, 0(t0)\n  sw t0, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Global("a".into()).move_content_to(ValueLocation::Global("b".into())),
            "  la t0, a\n  lw t0, 0(t0)\n  la t1, b\n  sw t0, 0(t1)"
        )
    }

    #[test]
    fn act_as_function_arg_test() {
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(0),
            "  li a0, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(1),
            "  li a1, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(2),
            "  li a2, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(3),
            "  li a3, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(4),
            "  li a4, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(5),
            "  li a5, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(6),
            "  li a6, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(7),
            "  li a7, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(8),
            "  li t0, 1\n  sw t0, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(9),
            "  li t0, 1\n  sw t0, 4(sp)"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(10),
            "  li t0, 1\n  sw t0, 8(sp)"
        );

        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(0),
            "  mv a0, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(1),
            "  mv a1, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(2),
            "  mv a2, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(3),
            "  mv a3, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(4),
            "  mv a4, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(5),
            "  mv a5, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(6),
            "  mv a6, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(7),
            "  mv a7, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(8),
            "  sw a0, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(9),
            "  sw a0, 4(sp)"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(10),
            "  sw a0, 8(sp)"
        );

        assert_eq!(
            ValueLocation::Stack("0(sp)".into(),).act_as_function_arg(0),
            "  lw a0, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into(),).act_as_function_arg(1),
            "  lw a1, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into(),).act_as_function_arg(2),
            "  lw a2, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into(),).act_as_function_arg(3),
            "  lw a3, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into(),).act_as_function_arg(4),
            "  lw a4, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into(),).act_as_function_arg(5),
            "  lw a5, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into(),).act_as_function_arg(6),
            "  lw a6, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into(),).act_as_function_arg(7),
            "  lw a7, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into(),).act_as_function_arg(8),
            "  lw t0, 0(sp)\n  sw t0, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into(),).act_as_function_arg(9),
            "  lw t0, 0(sp)\n  sw t0, 4(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into(),).act_as_function_arg(10),
            "  lw t0, 0(sp)\n  sw t0, 8(sp)"
        );

        assert_eq!(
            ValueLocation::Global("a".into()).act_as_function_arg(0),
            "  la t0, a\n  lw a0, 0(t0)"
        );
        assert_eq!(
            ValueLocation::Global("a".into()).act_as_function_arg(1),
            "  la t0, a\n  lw a1, 0(t0)"
        );
        assert_eq!(
            ValueLocation::Global("a".into()).act_as_function_arg(2),
            "  la t0, a\n  lw a2, 0(t0)"
        );
        assert_eq!(
            ValueLocation::Global("a".into()).act_as_function_arg(3),
            "  la t0, a\n  lw a3, 0(t0)"
        );
        assert_eq!(
            ValueLocation::Global("a".into()).act_as_function_arg(4),
            "  la t0, a\n  lw a4, 0(t0)"
        );
        assert_eq!(
            ValueLocation::Global("a".into()).act_as_function_arg(5),
            "  la t0, a\n  lw a5, 0(t0)"
        );
        assert_eq!(
            ValueLocation::Global("a".into()).act_as_function_arg(6),
            "  la t0, a\n  lw a6, 0(t0)"
        );
        assert_eq!(
            ValueLocation::Global("a".into()).act_as_function_arg(7),
            "  la t0, a\n  lw a7, 0(t0)"
        );
        assert_eq!(
            ValueLocation::Global("a".into()).act_as_function_arg(8),
            "  la t0, a\n  lw t0, 0(t0)\n  sw t0, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Global("a".into()).act_as_function_arg(9),
            "  la t0, a\n  lw t0, 0(t0)\n  sw t0, 4(sp)"
        );
        assert_eq!(
            ValueLocation::Global("a".into()).act_as_function_arg(10),
            "  la t0, a\n  lw t0, 0(t0)\n  sw t0, 8(sp)"
        );
    }
}
