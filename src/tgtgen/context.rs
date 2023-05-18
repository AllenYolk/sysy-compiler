use koopa::ir::entities::*;
use koopa::ir::*;

pub struct ProgramContext<'a> {
    pub program: &'a Program,
    // Function has implemented the Copy trait!
    pub func: Option<Function>,
}

impl<'a> ProgramContext<'a> {
    /// Construct a new program context.
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            func: None,
        }
    }

    /// Given a `Value` handler, return the corresponding `ValueData` in the current program context
    pub fn get_value_data(&self, val: Value) -> Option<&'a ValueData> {
        // `Value` has implemented the Copy trait!
        let Some(cur_func) = self.func else {
            return None;
        };
        let cur_func_data = self.program.func(cur_func);
        Some(cur_func_data.dfg().value(val))
    }
}
