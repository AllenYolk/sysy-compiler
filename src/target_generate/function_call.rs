use super::value_location::ValueLocation;

pub fn function_arg_location(i: usize, stack_frame_size: usize) -> ValueLocation {
    if i < 8 {
        ValueLocation::Reg(format!("a{}", i))
    } else {
        ValueLocation::Stack(format!("{}(sp)", 4 * (i - 8) + stack_frame_size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_arg_location_test() {
        assert_eq!(
            function_arg_location(0, 16),
            ValueLocation::Reg("a0".into())
        );
        assert_eq!(
            function_arg_location(1, 16),
            ValueLocation::Reg("a1".into())
        );
        assert_eq!(
            function_arg_location(2, 16),
            ValueLocation::Reg("a2".into())
        );
        assert_eq!(
            function_arg_location(3, 16),
            ValueLocation::Reg("a3".into())
        );
        assert_eq!(
            function_arg_location(4, 16),
            ValueLocation::Reg("a4".into())
        );
        assert_eq!(
            function_arg_location(5, 16),
            ValueLocation::Reg("a5".into())
        );
        assert_eq!(
            function_arg_location(6, 16),
            ValueLocation::Reg("a6".into())
        );
        assert_eq!(
            function_arg_location(7, 16),
            ValueLocation::Reg("a7".into())
        );
        assert_eq!(
            function_arg_location(8, 16),
            ValueLocation::Stack("16(sp)".into())
        );
        assert_eq!(
            function_arg_location(9, 16),
            ValueLocation::Stack("20(sp)".into())
        );
        assert_eq!(
            function_arg_location(10, 16),
            ValueLocation::Stack("24(sp)".into())
        );
    }
}
