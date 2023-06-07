use super::function_scan::*;
use super::value_location::*;
use koopa::ir::entities::*;

/// Context information used during RISC-V assembly generation.
pub struct ProgramContext<'a> {
    /// The current program.
    pub program: &'a Program,
    /// The current function.
    pub func: Option<FunctionScanResult>,
}

#[allow(dead_code)]
impl<'a> ProgramContext<'a> {
    /// Construct a new program context.
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            func: None,
        }
    }

    /// Get the `FunctionData` corresponding to the `func` field of the struct.
    pub fn get_current_function_data(&self) -> Option<&FunctionData> {
        let Some(FunctionScanResult{func: cur_func, .. }) = self.func else {
            return None;
        };
        Some(self.program.func(cur_func))
    }

    pub fn get_function_data(&self, func: Function) -> &FunctionData {
        self.program.func(func)
    }

    pub fn get_current_stack_frame_size(&self) -> Option<usize> {
        let Some(FunctionScanResult{stack_frame_size, .. }) = self.func else {
            return None;
        };
        Some(stack_frame_size)
    }

    /// Given a `Value` handler, return the corresponding `ValueData` in the current program context.
    pub fn get_value_data(&self, val: Value) -> Option<&ValueData> {
        // `Value` has implemented the Copy trait!
        let Some(cur_func_data) = self.get_current_function_data() else {
            return None;
        };
        Some(cur_func_data.dfg().value(val))
    }

    /// Given a `BasicBlock` handler, return the corresponding `BasicBlockData` in the current program context.
    pub fn get_basic_block_data(&self, bb: BasicBlock) -> Option<&BasicBlockData> {
        let Some(cur_func_data) = self.get_current_function_data() else {
            return None;
        };
        Some(cur_func_data.dfg().bb(bb))
    }

    pub fn get_value_location(&self, val: Value) -> Option<ValueLocation> {
        let Some(cur_func) = &self.func else {
            return None;
        };
        cur_func.value_locations.get(&val).map(|x| x.clone())
    }
}
