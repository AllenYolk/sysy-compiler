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
    /// A placeholder.
    ///
    /// Used in the implementation of `FunctionData.generate(...)`,
    /// in order to put the value location (returned by `cxt.get_value_location(Value)`) at a correct place.
    PlaceHolder(String),
    None,
}

impl ValueLocation {
    /// Generate the instruction (a `String`) that moves the value (with the location) to the given register.
    pub fn move_to_reg(&self, reg: &str) -> String {
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
            _ => String::new(),
        }
    }

    /// Generate the instruction (a `String`) that moves the value (with the location) to the given stack address.
    pub fn move_to_stack(&self, addr: &str) -> String {
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
            _ => String::new(),
        }
    }

    /// Generate the instruction (a `String`) that moves the value (with the location) to the given destination.
    pub fn move_to(&self, dest: ValueLocation) -> String {
        match dest {
            Self::Reg(r) => self.move_to_reg(&r),
            Self::Stack(addr) => self.move_to_stack(&addr),
            _ => String::new(),
        }
    }

    /// Generate the instruction (a `String`) that treat the value as the i-th function argument.
    pub fn act_as_function_arg(&self, i: usize, stack_frame_size: usize) -> String {
        let dest = function_arg_location(i, stack_frame_size);
        self.move_to(dest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_to_test() {
        assert_eq!(
            ValueLocation::Imm("1".into()).move_to(ValueLocation::Reg("a0".into())),
            "  li a0, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).move_to(ValueLocation::Stack("0(sp)".into())),
            "  li t0, 1\n  sw t0, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).move_to(ValueLocation::Reg("a1".into())),
            "  mv a1, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).move_to(ValueLocation::Stack("0(sp)".into())),
            "  sw a0, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into()).move_to(ValueLocation::Reg("a0".into())),
            "  lw a0, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into()).move_to(ValueLocation::Stack("4(sp)".into())),
            "  lw t0, 0(sp)\n  sw t0, 4(sp)"
        );
    }

    #[test]
    fn act_as_function_arg_test() {
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(0, 16),
            "  li a0, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(1, 16),
            "  li a1, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(2, 16),
            "  li a2, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(3, 16),
            "  li a3, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(4, 16),
            "  li a4, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(5, 16),
            "  li a5, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(6, 16),
            "  li a6, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(7, 16),
            "  li a7, 1"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(8, 16),
            "  li t0, 1\n  sw t0, 16(sp)"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(9, 16),
            "  li t0, 1\n  sw t0, 20(sp)"
        );
        assert_eq!(
            ValueLocation::Imm("1".into()).act_as_function_arg(10, 16),
            "  li t0, 1\n  sw t0, 24(sp)"
        );

        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(0, 16),
            "  mv a0, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(1, 16),
            "  mv a1, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(2, 16),
            "  mv a2, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(3, 16),
            "  mv a3, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(4, 16),
            "  mv a4, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(5, 16),
            "  mv a5, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(6, 16),
            "  mv a6, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(7, 16),
            "  mv a7, a0"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(8, 16),
            "  sw a0, 16(sp)"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(9, 16),
            "  sw a0, 20(sp)"
        );
        assert_eq!(
            ValueLocation::Reg("a0".into()).act_as_function_arg(10, 16),
            "  sw a0, 24(sp)"
        );

        assert_eq!(
            ValueLocation::Stack("0(sp)".into()).act_as_function_arg(0, 16),
            "  lw a0, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into()).act_as_function_arg(1, 16),
            "  lw a1, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into()).act_as_function_arg(2, 16),
            "  lw a2, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into()).act_as_function_arg(3, 16),
            "  lw a3, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into()).act_as_function_arg(4, 16),
            "  lw a4, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into()).act_as_function_arg(5, 16),
            "  lw a5, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into()).act_as_function_arg(6, 16),
            "  lw a6, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into()).act_as_function_arg(7, 16),
            "  lw a7, 0(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into()).act_as_function_arg(8, 16),
            "  lw t0, 0(sp)\n  sw t0, 16(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into()).act_as_function_arg(9, 16),
            "  lw t0, 0(sp)\n  sw t0, 20(sp)"
        );
        assert_eq!(
            ValueLocation::Stack("0(sp)".into()).act_as_function_arg(10, 16),
            "  lw t0, 0(sp)\n  sw t0, 24(sp)"
        );
    }
}
