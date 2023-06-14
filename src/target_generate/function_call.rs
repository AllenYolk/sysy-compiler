use super::value_location::ValueLocation;

/// Get the location of the i-th argument of a function.
/// 
/// If `call_another` is true, find the location of the parameter of another function being called now.
/// If false, find the location of the parameter of the current function.
pub fn function_arg_location(i: usize, stack_frame_size: usize, call_another: bool) -> ValueLocation {
    if i < 8 {
        ValueLocation::Reg(format!("a{}", i))
    } else {
        if call_another {
            ValueLocation::Stack(format!("{}(sp)", 4 * (i - 8)))
        } else {
            ValueLocation::Stack(format!("{}(sp)", 4 * (i - 8) + stack_frame_size))
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_arg_location_test() {
        assert_eq!(
            function_arg_location(0, 16, true),
            ValueLocation::Reg("a0".into())
        );
        assert_eq!(
            function_arg_location(1, 16, false),
            ValueLocation::Reg("a1".into())
        );
        assert_eq!(
            function_arg_location(2, 16, false),
            ValueLocation::Reg("a2".into())
        );
        assert_eq!(
            function_arg_location(3, 16, true),
            ValueLocation::Reg("a3".into())
        );
        assert_eq!(
            function_arg_location(4, 16, true),
            ValueLocation::Reg("a4".into())
        );
        assert_eq!(
            function_arg_location(5, 16, false),
            ValueLocation::Reg("a5".into())
        );
        assert_eq!(
            function_arg_location(6, 16, false),
            ValueLocation::Reg("a6".into())
        );
        assert_eq!(
            function_arg_location(7, 16, false),
            ValueLocation::Reg("a7".into())
        );

        assert_eq!(
            function_arg_location(8, 16, false),
            ValueLocation::Stack("16(sp)".into())
        );
        assert_eq!(
            function_arg_location(9, 16, false),
            ValueLocation::Stack("20(sp)".into())
        );
        assert_eq!(
            function_arg_location(10, 16, false),
            ValueLocation::Stack("24(sp)".into())
        );

        assert_eq!(
            function_arg_location(8, 16, true),
            ValueLocation::Stack("0(sp)".into())
        );
        assert_eq!(
            function_arg_location(9, 16, true),
            ValueLocation::Stack("4(sp)".into())
        );
        assert_eq!(
            function_arg_location(10, 16, true),
            ValueLocation::Stack("8(sp)".into())
        );
    }
}
