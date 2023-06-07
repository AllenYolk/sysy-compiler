use koopa::ir::entities::*;
use koopa::ir::*;
use std::collections::HashMap;
use super::value_location::*;

/// The result of function scanning.
pub struct FunctionScanResult {
    /// The handle of the function.
    /// 
    /// Notice that `Function` has implemented the `Copy` trait!
    pub func: Function,
    /// The locations of the values in the function.
    /// 
    /// During scanning, the addresses of all the variables are decided.
    /// Notice that `Value` has implemented the `Copy` trait!
    pub value_locations: HashMap<Value, ValueLocation>,
    /// The size of the stack frame.
    /// 
    /// This value has not be ceiled up to 16 bytes.
    pub stack_frame_size: usize,
    /// The location of the return address slot.
    /// 
    /// `None` if the function does not call other functions.
    pub ra_slot_location: Option<ValueLocation>,
}

impl FunctionScanResult {
    /// Scan the function and yield a `FunctionScanResult`.
    pub fn try_from(func: Function, func_data: &FunctionData) -> Result<Self, ()> {
        let mut n_local_var = 0usize;
        let mut n_param_on_stack = 0usize;
        let mut has_call = false;
        let mut value_slots = HashMap::new();
        func_data.scan(&mut value_slots, &mut n_local_var, &mut n_param_on_stack, &mut has_call)?;

        let stack_frame_size = (n_param_on_stack + n_local_var + (has_call as usize)) * 4;
        let mut ra_slot_location = None;
        if has_call {
            ra_slot_location = Some(ValueLocation::Stack(format!("{}(sp)", stack_frame_size - 4)));
        }
        let value_locations: HashMap<Value, ValueLocation> = value_slots.into_iter().map(|(k, v)| {
            let loc = 4 * (v + n_param_on_stack);
            (k, ValueLocation::Stack(format!("{}(sp)", loc)))
        }).collect();
        
        Ok(Self { func, value_locations, stack_frame_size, ra_slot_location })
    }
}

/// Scan the function and update some fields.
/// 
/// The implementation should update the following fields:
/// * `value_slots`: the slot id (order number) of each value. This field should be updated by `FunctionData`'s implementation only!
/// * `n_local_var`: the number of local variables.
/// * `n_param_on_stack`: the number of function parameters on the stack. Updated only by the implementation of `Call`.
/// * `has_call`: whether the function calls other functions. Updated only by the implementation of `Call`.
trait FunctionScan {
    type Ret;

    fn scan(&self, value_slots: &mut HashMap<Value, usize>, n_local_var: &mut usize, n_param_on_stack: &mut usize, has_call: &mut bool) -> Result<Self::Ret, ()>;
}

impl FunctionScan for FunctionData {
    type Ret = ();

    fn scan(&self, value_slots: &mut HashMap<Value, usize>, n_local_var: &mut usize, n_param_on_stack: &mut usize, has_call: &mut bool) -> Result<Self::Ret, ()> {
        for (_bb, node) in self.layout().bbs() {
            for &inst_val in node.insts().keys() {
                let inst_val_data = self.dfg().value(inst_val);
                let loc = inst_val_data.scan(value_slots, n_local_var, n_param_on_stack, has_call)?;
                match loc {
                    Some(o) => {
                        value_slots.insert(inst_val, o);
                    }
                    None => (),
                }
            }
        }

        Ok(())
    }
}

impl FunctionScan for ValueData {
    type Ret = Option<usize>;

    fn scan(&self, value_slots: &mut HashMap<Value, usize>, n_local_var: &mut usize, n_param_on_stack: &mut usize, has_call: &mut bool) -> Result<Self::Ret, ()> {
        match self.kind() {
            // integer constant
            ValueKind::Integer(val) => val.scan(value_slots, n_local_var, n_param_on_stack, has_call),
            // allocation operation
            ValueKind::Alloc(val) => val.scan(value_slots, n_local_var, n_param_on_stack, has_call),
            // load operation
            ValueKind::Load(val) => val.scan(value_slots, n_local_var, n_param_on_stack, has_call),
            // store operation
            ValueKind::Store(val) => val.scan(value_slots, n_local_var, n_param_on_stack, has_call),
            // binary operation
            ValueKind::Binary(val) => val.scan(value_slots, n_local_var, n_param_on_stack, has_call),
            // branch operation
            ValueKind::Branch(val) => val.scan(value_slots, n_local_var, n_param_on_stack, has_call),
            // jump operation
            ValueKind::Jump(val) => val.scan(value_slots, n_local_var, n_param_on_stack, has_call),
            // function return
            ValueKind::Return(val) => val.scan(value_slots, n_local_var, n_param_on_stack, has_call),
            // others
            _ => Err(()),
        }
    }
}

impl FunctionScan for values::Integer {
    type Ret = Option<usize>;

    fn scan(&self, _value_slots: &mut HashMap<Value, usize>, _n_local_var: &mut usize, _n_param_on_stack: &mut usize, _has_call: &mut bool) -> Result<Self::Ret, ()> {
        Ok(None)
    }
}

impl FunctionScan for values::Alloc {
    type Ret = Option<usize>;

    fn scan(&self, _value_slots: &mut HashMap<Value, usize>, n_local_var: &mut usize, _n_param_on_stack: &mut usize, _has_call: &mut bool) -> Result<Self::Ret, ()> {
        *n_local_var += 1;
        Ok(Some(*n_local_var - 1))
    }
}

impl FunctionScan for values::Load {
    type Ret = Option<usize>;

    fn scan(&self, _value_slots: &mut HashMap<Value, usize>, n_local_var: &mut usize, _n_param_on_stack: &mut usize, _has_call: &mut bool) -> Result<Self::Ret, ()> {
        *n_local_var += 1;
        Ok(Some(*n_local_var - 1))
    }
}

impl FunctionScan for values::Store {
    type Ret = Option<usize>;

    fn scan(&self, _value_slots: &mut HashMap<Value, usize>, _n_local_var: &mut usize, _n_param_on_stack: &mut usize, _has_call: &mut bool) -> Result<Self::Ret, ()> {
        Ok(None)
    }
}

impl FunctionScan for values::Binary {
    type Ret = Option<usize>;

    fn scan(&self, _value_slots: &mut HashMap<Value, usize>, n_local_var: &mut usize, _n_param_on_stack: &mut usize, _has_call: &mut bool) -> Result<Self::Ret, ()> {
        *n_local_var += 1;
        Ok(Some(*n_local_var - 1))
    }
}

impl FunctionScan for values::Branch {
    type Ret = Option<usize>;

    fn scan(&self, _value_slots: &mut HashMap<Value, usize>, _n_local_var: &mut usize, _n_param_on_stack: &mut usize, _has_call: &mut bool) -> Result<Self::Ret, ()> {
        Ok(None)
    }
}

impl FunctionScan for values::Jump {
    type Ret = Option<usize>;

    fn scan(&self, _value_slots: &mut HashMap<Value, usize>, _n_local_var: &mut usize, _n_param_on_stack: &mut usize, _has_call: &mut bool) -> Result<Self::Ret, ()> {
        Ok(None)
    }
}

impl FunctionScan for values::Return {
    type Ret = Option<usize>;

    fn scan(&self, _value_slots: &mut HashMap<Value, usize>, _n_local_var: &mut usize, _n_param_on_stack: &mut usize, _has_call: &mut bool) -> Result<Self::Ret, ()> {
        Ok(None)
    }
}