use super::value_location::*;
use koopa::ir::entities::*;
use std::collections::HashMap;

/// Context information used during RISC-V assembly generation.
pub struct ProgramContext<'a> {
    pub program: &'a Program,
    // `Function` has implemented the Copy trait!
    pub func: Option<Function>,
    // `Value` has implemented the Copy trait!
    value_locations: HashMap<Value, ValueLocation>,
    offset: usize,
}

#[allow(dead_code)]
impl<'a> ProgramContext<'a> {
    /// Construct a new program context.
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            func: None,
            value_locations: HashMap::new(),
            offset: 0,
        }
    }

    /// Get the `FunctionData` corresponding to the `func` field of the struct.
    pub fn get_current_function_data(&self) -> Option<&FunctionData> {
        let Some(cur_func) = self.func else {
            return None;
        };
        Some(self.program.func(cur_func))
    }

    /// Given a `Value` handler, return the corresponding `ValueData` in the current program context
    pub fn get_value_data(&self, val: Value) -> Option<&ValueData> {
        // `Value` has implemented the Copy trait!
        let Some(cur_func_data) = self.get_current_function_data() else {
            return None;
        };
        Some(cur_func_data.dfg().value(val))
    }

    pub fn get_basic_block_data(&self, bb: BasicBlock) -> Option<&BasicBlockData> {
        let Some(cur_func_data) = self.get_current_function_data() else {
            return None;
        };
        Some(cur_func_data.dfg().bb(bb))
    }

    pub fn reset_offset(&mut self) {
        self.offset = 0;
    }

    pub fn total_offset(&self) -> usize {
        self.offset
    }

    /// Allocate a new local variable (with a certain size) to an empty slot in the current stack frame.
    ///
    /// The offset w.r.t. the stack frame pointer is returned.
    pub fn alloc_local_stack_variable(&mut self, size: usize) -> usize {
        let res = self.offset;
        self.offset += size;
        res
    }

    /// Set the location (`ValueLocation`) of a value (`Value`).
    ///
    /// The key-value pair is inserted into the `value_locations` field, which is a `HashMap`.
    pub fn set_value_location(&mut self, val: Value, loc: ValueLocation) -> Option<ValueLocation> {
        self.value_locations.insert(val, loc)
    }

    /// Get the location (`ValueLocation`) of a value (`Value`).
    ///
    /// Just look up the hashmap `value_locations`.
    pub fn get_value_location(&self, val: Value) -> Option<&ValueLocation> {
        self.value_locations.get(&val)
    }
}
