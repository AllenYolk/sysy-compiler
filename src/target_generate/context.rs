use super::function_scan::*;
use super::value_location::*;
use koopa::ir::entities::*;
use std::collections::HashMap;

/// Context information used during RISC-V assembly generation.
pub struct ProgramContext<'a> {
    /// The current program.
    pub program: &'a Program,
    /// The current function.
    pub func: Option<FunctionScanResult>,
    pub global_values: HashMap<Value, ValueLocation>,
}

#[allow(dead_code)]
impl<'a> ProgramContext<'a> {
    /// Construct a new program context.
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            func: None,
            global_values: HashMap::new(),
        }
    }

    /// Get the `FunctionData` corresponding to the `func` field of the struct.
    pub fn get_current_function_data(&self) -> Option<&FunctionData> {
        let Some(FunctionScanResult{func: cur_func, .. }) = self.func else {
            return None;
        };
        Some(self.program.func(cur_func))
    }

    /// Get the `FunctionData` corresponding to the given `Function` handler.
    pub fn get_function_data(&self, func: Function) -> &FunctionData {
        self.program.func(func)
    }

    /// Get the stack frame size of the current function.
    ///
    /// 1. The current function is stored in `self.func`.
    /// 2. The return value has been ceiled up to 16 bytes.
    pub fn get_current_stack_frame_size(&self) -> Option<usize> {
        let Some(FunctionScanResult{stack_frame_size, .. }) = self.func else {
            return None;
        };
        Some(stack_frame_size)
    }

    /// Given a `Value` handler, return the corresponding `ValueData` in the current program context or that of a global value.
    ///
    /// The order of searching does not matter, since the `Value` handler is unique.
    pub fn get_value_data_locally_or_globally(&self, val: Value) -> Option<ValueData> {
        // `Value` has implemented the Copy trait!
        if self.global_values.contains_key(&val) {
            // The value is a global value.
            return Some(self.program.borrow_value(val).clone());
        }

        let Some(cur_func_data) = self.get_current_function_data() else {
            return None;
        };
        Some(cur_func_data.dfg().value(val).clone())
    }

    /// Given a `BasicBlock` handler, return the corresponding `BasicBlockData` in the current program context.
    ///
    /// The basic block is local to certain function (stored in `self.func`).
    pub fn get_basic_block_data_in_current_function(
        &self,
        bb: BasicBlock,
    ) -> Option<&BasicBlockData> {
        let Some(cur_func_data) = self.get_current_function_data() else {
            return None;
        };
        Some(cur_func_data.dfg().bb(bb))
    }

    /// Given a `Value`, get its location in the "current function" or that of a global value.
    ///
    /// First, search in the current function. If the value is not found, search in the global values.
    /// Actually, the order of searching does not matter, since the `Value` handler is unique.
    pub fn get_value_location_local_or_global(&self, val: Value) -> Option<ValueLocation> {
        let Some(cur_func) = &self.func else {
            return None;
        };
        let op_loc = cur_func.value_locations.get(&val);
        match op_loc {
            Some(loc) => Some(loc.clone()),
            None => self.global_values.get(&val).map(|loc| loc.clone()),
        }
    }

    pub fn add_global_value(&mut self, val: Value, loc: ValueLocation) -> Result<(), ()> {
        if let Some(_) = self.global_values.insert(val, loc) {
            return Err(());
        };
        Ok(())
    }
}
