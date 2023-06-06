use std::collections::HashMap;

#[derive(Clone)]
pub enum SymbolTableValue {
    Const(String),
    Var(String),
}

#[derive(Clone)]
pub struct LoopLabel {
    pub entry: String,
    pub body: String,
    pub end: String,
}

#[allow(dead_code)]
pub struct Scopes {
    /// identifier -> koopa symbol name
    functions: HashMap<String, String>,
    /// stacked symbol tables
    /// identifier -> koopa symbol name
    values: Vec<HashMap<String, SymbolTableValue>>,
    /// stacked loop information
    loops: Vec<LoopLabel>,
}

#[allow(dead_code)]
impl Scopes {
    /// Construct a new `Scopes` record.
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            values: vec![HashMap::new()],
            loops: Vec::new(),
        }
    }

    pub fn get_function(&self, identifier: &str) -> Result<String, ()> {
        let Some(res) = self.functions.get(identifier) else {
            return Err(());
        };
        Ok(res.clone())
    }

    pub fn add_function(&mut self, identifier: &str, symbol: &str) -> Result<(), ()> {
        if let Some(_) = self.functions.insert(identifier.into(), symbol.into()) {
            return Err(());
        };
        Ok(())
    }

    /// Enter a new scope.
    pub fn enter(&mut self) {
        self.values.push(HashMap::new());
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
    pub fn add_value(&mut self, identifier: &str, symbol: &str, is_const: bool) -> Result<(), ()> {
        let Some(symtab) = self.values.last_mut() else {
            return Err(());
        };
        let v = if is_const {
            SymbolTableValue::Const(symbol.into())
        } else {
            SymbolTableValue::Var(symbol.into())
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
}
