/// A struct describing where a value is located.
#[derive(Clone)]
#[allow(dead_code)]
pub enum ValueLocation {
    Imm(String),
    Reg(String),
    Stack(String),
    None,
}

impl ValueLocation {
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
            Self::None => String::new(),
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
