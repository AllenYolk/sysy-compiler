use crate::tools::ceil_to_k;
use koopa::ir::entities::*;
use koopa::ir::*;
use std::collections::HashMap;

use super::function_call::function_arg_location;
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
    /// Each of them has a **unique** location on the stack!
    /// Notice that `Value` has implemented the `Copy` trait!
    pub value_locations: HashMap<Value, ValueLocation>,
    /// Whether the `Value`'s `ValueLocation` contains a pointer to:
    /// 1. the data that a Koopa symbol refers to
    /// or
    /// 2. the data that a Koopa pointer points to
    /// rather than containing these data themselves.
    ///
    /// If true, we cannot load or store the `Value` directly.
    pub contain_pointer: HashMap<Value, bool>,
    /// The size of the stack frame.
    ///
    /// This value has been ceiled up to 16 bytes.
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
        let mut contain_pointer = HashMap::new();
        func_data.scan(
            &mut value_slots,
            &mut contain_pointer,
            &mut n_local_var,
            &mut n_param_on_stack,
            &mut has_call,
            None,
        )?;

        let stack_frame_size = ceil_to_k(
            (n_param_on_stack + n_local_var + (has_call as usize)) * 4,
            16usize,
        );
        let mut ra_slot_location = None;
        if has_call {
            ra_slot_location = Some(ValueLocation::Stack(format!(
                "{}(sp)",
                stack_frame_size - 4
            )));
        }
        let mut value_locations: HashMap<Value, ValueLocation> = value_slots
            .into_iter()
            .map(|(k, v)| {
                let loc = 4 * (v + n_param_on_stack);
                (k, ValueLocation::Stack(format!("{}(sp)", loc)))
            })
            .collect();
        // we have to add the function parameters to the `value_locations` map
        for (i, param) in func_data.params().iter().enumerate() {
            value_locations.insert(*param, function_arg_location(i, stack_frame_size));
        }

        Ok(Self {
            func,
            value_locations,
            contain_pointer,
            stack_frame_size,
            ra_slot_location,
        })
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

    fn scan(
        &self,
        value_slots: &mut HashMap<Value, usize>,
        contain_pointer: &mut HashMap<Value, bool>,
        n_local_var: &mut usize,
        n_param_on_stack: &mut usize,
        has_call: &mut bool,
        value_data: Option<&ValueData>,
    ) -> Result<Self::Ret, ()>;
}

impl FunctionScan for FunctionData {
    type Ret = ();

    fn scan(
        &self,
        value_slots: &mut HashMap<Value, usize>,
        contain_pointer: &mut HashMap<Value, bool>,
        n_local_var: &mut usize,
        n_param_on_stack: &mut usize,
        has_call: &mut bool,
        _value_data: Option<&ValueData>,
    ) -> Result<Self::Ret, ()> {
        for (_bb, node) in self.layout().bbs() {
            for &inst_val in node.insts().keys() {
                let inst_val_data = self.dfg().value(inst_val);
                let loc = inst_val_data.scan(
                    value_slots,
                    contain_pointer,
                    n_local_var,
                    n_param_on_stack,
                    has_call,
                    None,
                )?;
                match loc {
                    Some(o) => {
                        // the instruction yields a new value
                        value_slots.insert(inst_val, o);
                        match inst_val_data.kind() {
                            ValueKind::GetElemPtr(_) => {
                                contain_pointer.insert(inst_val, true);
                            }
                            _ => {
                                contain_pointer.insert(inst_val, false);
                            }
                        }
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

    fn scan(
        &self,
        value_slots: &mut HashMap<Value, usize>,
        contain_pointer: &mut HashMap<Value, bool>,
        n_local_var: &mut usize,
        n_param_on_stack: &mut usize,
        has_call: &mut bool,
        _value_data: Option<&ValueData>,
    ) -> Result<Self::Ret, ()> {
        match self.kind() {
            ValueKind::Alloc(val) => val.scan(
                value_slots,
                contain_pointer,
                n_local_var,
                n_param_on_stack,
                has_call,
                Some(self),
            ),
            ValueKind::Load(val) => val.scan(
                value_slots,
                contain_pointer,
                n_local_var,
                n_param_on_stack,
                has_call,
                Some(self),
            ),
            ValueKind::Store(val) => val.scan(
                value_slots,
                contain_pointer,
                n_local_var,
                n_param_on_stack,
                has_call,
                Some(self),
            ),
            ValueKind::GetElemPtr(val) => val.scan(
                value_slots,
                contain_pointer,
                n_local_var,
                n_param_on_stack,
                has_call,
                Some(self),
            ),
            ValueKind::Binary(val) => val.scan(
                value_slots,
                contain_pointer,
                n_local_var,
                n_param_on_stack,
                has_call,
                Some(self),
            ),
            ValueKind::Branch(val) => val.scan(
                value_slots,
                contain_pointer,
                n_local_var,
                n_param_on_stack,
                has_call,
                Some(self),
            ),
            ValueKind::Jump(val) => val.scan(
                value_slots,
                contain_pointer,
                n_local_var,
                n_param_on_stack,
                has_call,
                Some(self),
            ),
            ValueKind::Call(val) => val.scan(
                value_slots,
                contain_pointer,
                n_local_var,
                n_param_on_stack,
                has_call,
                Some(self),
            ),
            ValueKind::Return(val) => val.scan(
                value_slots,
                contain_pointer,
                n_local_var,
                n_param_on_stack,
                has_call,
                Some(self),
            ),

            // others: unreachable
            _ => Err(()),
        }
    }
}

impl FunctionScan for values::Alloc {
    type Ret = Option<usize>;

    fn scan(
        &self,
        _value_slots: &mut HashMap<Value, usize>,
        _contain_pointer: &mut HashMap<Value, bool>,
        n_local_var: &mut usize,
        _n_param_on_stack: &mut usize,
        _has_call: &mut bool,
        value_data: Option<&ValueData>,
    ) -> Result<Self::Ret, ()> {
        let Some(value_data) = value_data else {
            return Err(());
        };
        let old_n_local_var = *n_local_var;

        let target_size = match value_data.ty().kind() {
            TypeKind::Pointer(base) => base.size(),
            _ => value_data.ty().size(),
        };
        let n_var = target_size / 4;
        *n_local_var += n_var;
        Ok(Some(old_n_local_var))
    }
}

impl FunctionScan for values::Load {
    type Ret = Option<usize>;

    fn scan(
        &self,
        _value_slots: &mut HashMap<Value, usize>,
        _contain_pointer: &mut HashMap<Value, bool>,
        n_local_var: &mut usize,
        _n_param_on_stack: &mut usize,
        _has_call: &mut bool,
        _value_data: Option<&ValueData>,
    ) -> Result<Self::Ret, ()> {
        *n_local_var += 1;
        Ok(Some(*n_local_var - 1))
    }
}

impl FunctionScan for values::Store {
    type Ret = Option<usize>;

    fn scan(
        &self,
        _value_slots: &mut HashMap<Value, usize>,
        _contain_pointer: &mut HashMap<Value, bool>,
        _n_local_var: &mut usize,
        _n_param_on_stack: &mut usize,
        _has_call: &mut bool,
        _value_data: Option<&ValueData>,
    ) -> Result<Self::Ret, ()> {
        Ok(None)
    }
}

impl FunctionScan for values::GetElemPtr {
    type Ret = Option<usize>;

    fn scan(
        &self,
        _value_slots: &mut HashMap<Value, usize>,
        _contain_pointer: &mut HashMap<Value, bool>,
        n_local_var: &mut usize,
        _n_param_on_stack: &mut usize,
        _has_call: &mut bool,
        _value_data: Option<&ValueData>,
    ) -> Result<Self::Ret, ()> {
        *n_local_var += 1;
        Ok(Some(*n_local_var - 1))
    }
}

impl FunctionScan for values::Binary {
    type Ret = Option<usize>;

    fn scan(
        &self,
        _value_slots: &mut HashMap<Value, usize>,
        _contain_pointer: &mut HashMap<Value, bool>,
        n_local_var: &mut usize,
        _n_param_on_stack: &mut usize,
        _has_call: &mut bool,
        _value_data: Option<&ValueData>,
    ) -> Result<Self::Ret, ()> {
        *n_local_var += 1;
        Ok(Some(*n_local_var - 1))
    }
}

impl FunctionScan for values::Branch {
    type Ret = Option<usize>;

    fn scan(
        &self,
        _value_slots: &mut HashMap<Value, usize>,
        _contain_pointer: &mut HashMap<Value, bool>,
        _n_local_var: &mut usize,
        _n_param_on_stack: &mut usize,
        _has_call: &mut bool,
        _value_data: Option<&ValueData>,
    ) -> Result<Self::Ret, ()> {
        Ok(None)
    }
}

impl FunctionScan for values::Jump {
    type Ret = Option<usize>;

    fn scan(
        &self,
        _value_slots: &mut HashMap<Value, usize>,
        _contain_pointer: &mut HashMap<Value, bool>,
        _n_local_var: &mut usize,
        _n_param_on_stack: &mut usize,
        _has_call: &mut bool,
        _value_data: Option<&ValueData>,
    ) -> Result<Self::Ret, ()> {
        Ok(None)
    }
}

impl FunctionScan for values::Call {
    type Ret = Option<usize>;

    fn scan(
        &self,
        _value_slots: &mut HashMap<Value, usize>,
        _contain_pointer: &mut HashMap<Value, bool>,
        n_local_var: &mut usize,
        n_param_on_stack: &mut usize,
        has_call: &mut bool,
        _value_data: Option<&ValueData>,
    ) -> Result<Self::Ret, ()> {
        *has_call = *has_call || true;

        let n_args = self.args().len();
        let on_stack = n_args.checked_sub(8).unwrap_or(0);
        *n_param_on_stack = std::cmp::max(*n_param_on_stack, on_stack);

        // Whether the function returns a value or not, we allocate a slot for the return value.
        *n_local_var += 1;
        Ok(Some(*n_local_var - 1))
    }
}

impl FunctionScan for values::Return {
    type Ret = Option<usize>;

    fn scan(
        &self,
        _value_slots: &mut HashMap<Value, usize>,
        _contain_pointer: &mut HashMap<Value, bool>,
        _n_local_var: &mut usize,
        _n_param_on_stack: &mut usize,
        _has_call: &mut bool,
        _value_data: Option<&ValueData>,
    ) -> Result<Self::Ret, ()> {
        Ok(None)
    }
}
