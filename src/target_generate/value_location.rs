/// The location of a value.
///
/// The `String` contained in the variants can be used directly in RISC-V instructions!
#[derive(Clone)]
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_location_test() {
        let s1 = ValueLocation::Imm("114".into()).move_to_reg("a0");
        assert_eq!(s1, "  li a0, 114");

        let s2 = ValueLocation::Stack("16(sp)".into()).move_to_reg("t1");
        assert_eq!(s2, "  lw t1, 16(sp)");

        let s3 = ValueLocation::Reg("t1".into()).move_to_reg("a0");
        assert_eq!(s3, "  mv a0, t1");

        let s1 = ValueLocation::None.move_to_reg("a0");
        assert!(s1.is_empty());
    }
}
