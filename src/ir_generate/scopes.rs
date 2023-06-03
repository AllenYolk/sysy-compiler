use std::collections::HashMap;

#[derive(Clone)]
pub enum SymbolTableValue {
    Const(String),
    Var(String),
}

impl SymbolTableValue {
    pub fn is_const(&self) -> bool {
        match self {
            SymbolTableValue::Const(_) => true,
            SymbolTableValue::Var(_) => false,
        }
    }

    pub fn get_value(&self) -> String {
        match self {
            SymbolTableValue::Const(s) => s.clone(),
            SymbolTableValue::Var(s) => s.clone(),
        }
    }
}

#[allow(dead_code)]
pub struct Scopes {
    functions: HashMap<String, String>, // identifier -> koopa symbol name
    values: Vec<HashMap<String, SymbolTableValue>>, // identifier -> koopa symbol name
}

#[allow(dead_code)]
impl Scopes {
    /// Construct a new `Scopes` record.
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            values: vec![HashMap::new()],
        }
    }

    /// Enter a new scope.
    pub fn enter(&mut self) {
        self.values.push(HashMap::new());
    }

    /// Exit the current scopes.
    pub fn exit(&mut self) {
        self.values.pop();
    }

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
}
