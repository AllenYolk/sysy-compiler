use std::collections::HashMap;

/// Information about a function that will be used during Koopa text generation.
#[derive(Clone)]
pub struct FunctionInfo {
    pub symbol: String,
    pub return_void: bool,
    pub array_param: Vec<bool>,
    // there's no need to store detailed parameter types!!!
}

/// The value of a symbol in the symbol table.
///
/// In a symbol table (an element of the `values` field of the struct `Scopes`),
/// a value identifier (a `String`) is mapped to a symbol, which is an instance of this struct.
/// A symbol can be either a constant (a `String` indicating its literal value)
/// or a variable (a `String` representing a Koopa symbol),
/// or an array (constant and variable arrays are treated equally).
#[derive(Clone)]
pub enum SymbolTableValue {
    Const(String),
    Var(String),
    Array(String, usize),
}

/// The three labels defined for a `while` loop.
#[derive(Clone)]
pub struct LoopLabel {
    pub entry: String,
    pub body: String,
    pub end: String,
}

#[allow(dead_code)]
pub struct Scopes {
    /// All the functions defined in the program.
    ///
    /// identifier -> koopa symbol name
    functions: HashMap<String, FunctionInfo>,
    /// Stacked symbol tables.
    ///
    /// identifier -> koopa symbol name / const value
    values: Vec<HashMap<String, SymbolTableValue>>,
    /// Contents of `values_buffer` will be inserted into the scope entered next time.
    ///
    /// This field is used to put function parameters into the symbol table of the function body.
    values_buffer: Vec<(String, SymbolTableValue)>,
    /// Stacked loop information.
    loops: Vec<LoopLabel>,
    /// The parameter list of the current function.
    cur_func_params: Vec<String>,
}

#[allow(dead_code)]
impl Scopes {
    /// Construct a new `Scopes` record.
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            values: vec![HashMap::new()],
            values_buffer: Vec::new(),
            loops: Vec::new(),
            cur_func_params: Vec::new(),
        }
    }

    pub fn get_function(&self, identifier: &str) -> Result<FunctionInfo, ()> {
        let Some(res) = self.functions.get(identifier) else {
            return Err(());
        };
        Ok(res.clone())
    }

    pub fn add_function(
        &mut self,
        identifier: &str,
        symbol: &str,
        return_void: bool,
        array_param: Vec<bool>,
    ) -> Result<(), ()> {
        if let Some(_) = self.functions.insert(
            identifier.into(),
            FunctionInfo {
                symbol: symbol.into(),
                return_void,
                array_param,
            },
        ) {
            return Err(());
        };
        Ok(())
    }

    pub fn add_value_to_buffer(&mut self, identifier: &str, symbol: &str, is_const: bool, n_array_dim: Option<usize>) {
        let v = if let Some(nd) = n_array_dim {
            SymbolTableValue::Array(symbol.into(), nd)
        } else {
            if is_const {
                SymbolTableValue::Const(symbol.into())
            } else {
                SymbolTableValue::Var(symbol.into())
            }
        };
        self.values_buffer.push((identifier.into(), v));
    }

    /// Enter a new scope.
    ///
    /// Contents in `values_buffer` will be added to the new scope.
    pub fn enter(&mut self) -> Result<(), ()> {
        let mut map: HashMap<String, SymbolTableValue> = HashMap::new();
        for (k, v) in self.values_buffer.iter() {
            if let Some(_) = map.insert(k.clone(), v.clone()) {
                return Err(());
            };
        }
        self.values.push(map);
        self.values_buffer.clear();
        Ok(())
    }

    /// Exit the current scopes.
    pub fn exit(&mut self) {
        self.values.pop();
    }

    /// Get the value (symbol name) of the given identifier.
    ///
    /// If the identifier is not found in the current scope,
    /// search in the outer one, and then in the outer one of the outer one, and so on.
    /// `Err(())` is returned if the identifier is not found in all the scopes.
    pub fn get_value(&self, identifier: &str) -> Result<SymbolTableValue, ()> {
        let l = self.values.len();
        for i_ in 1..=l {
            let i = l - i_;
            if let Some(res) = self.values[i].get(identifier) {
                return Ok(res.clone());
            }
        }
        Err(())
    }

    /// Add a new value to the current scope.
    ///
    /// The entry is added to the symbol table at the top of the stack.
    pub fn add_value(
        &mut self,
        identifier: &str,
        symbol: &str,
        is_const: bool,
        n_array_dim: Option<usize>,
    ) -> Result<(), ()> {
        let Some(symtab) = self.values.last_mut() else {
            return Err(());
        };

        let v = if let Some(nd) = n_array_dim {
            SymbolTableValue::Array(symbol.into(), nd)
        } else {
            if is_const {
                SymbolTableValue::Const(symbol.into())
            } else {
                SymbolTableValue::Var(symbol.into())
            }
        };
        if let Some(_) = symtab.insert(identifier.into(), v) {
            return Err(()); // defined multiple times
        };
        Ok(())
    }

    /// Enter a new loop with the given labels.
    pub fn enter_loop(&mut self, entry_label: &str, body_label: &str, end_label: &str) {
        self.loops.push(LoopLabel {
            entry: entry_label.to_string(),
            body: body_label.to_string(),
            end: end_label.to_string(),
        })
    }

    /// Exit the current loop.
    pub fn exit_loop(&mut self) {
        self.loops.pop();
    }

    /// Get the labels for the current loop.
    ///
    /// The current loop information is located at the top of the stack.
    pub fn get_cur_loop_labels(&self) -> Result<LoopLabel, ()> {
        let Some(res) = self.loops.last() else {
            return Err(());
        };
        Ok(res.clone())
    }

    /// Return whether we're now in the global scope.
    ///
    /// This method can help us distinguishing local declarations from global ones!
    pub fn now_global(&self) -> bool {
        self.values.len() <= 1
    }

    /// Add a record of a parameter of the current function.
    /// 
    /// The record is stored in the `cur_func_params` field, which is a `Vec<String>`.
    /// Each record should be a Koopa symbol name, rather than a SysY identifier.
    pub fn add_cur_func_param(&mut self, param: &str) {
        self.cur_func_params.push(param.into());
    }

    pub fn clear_cur_func_params(&mut self) {
        self.cur_func_params.clear();
    }

    pub fn has_cur_func_param(&self, param: &str) -> bool {
        self.cur_func_params.contains(&param.into())
    }
}
