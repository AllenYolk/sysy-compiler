use std::collections::HashMap;

#[allow(dead_code)]
pub struct Scopes<'a> {
    functions: HashMap<&'a str, &'a str>, // identifier -> koopa symbol name
    values: Vec<HashMap<&'a str, &'a str>>, // identifier -> koopa symbol name
}

#[allow(dead_code)]
impl<'a> Scopes<'a> {
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

    pub fn get_value(&self, identifier: &str) -> Result<&str, ()> {
        let l = self.values.len();
        for i_ in 1..=l {
            let i = l - i_;
            if let Some(&res) = self.values[i].get(identifier) {
                return Ok(res);
            }
        }
        Err(())
    }

    pub fn add_value(&mut self, identifier: &'a str, symbol: &'a str) -> Result<(), ()> {
        let Some(symtab) = self.values.last_mut() else {
            return Err(());
        };
        if let Some(_) = symtab.insert(identifier, symbol) {
            return Err(());
        };
        Ok(())
    }

    pub fn get_function(&self, identifier: &str) -> Result<&str, ()> {
        let Some(&res) = self.functions.get(identifier) else {
            return Err(());
        };
        Ok(res)
    }

    pub fn add_function(&mut self, identifier: &'a str, symbol: &'a str) -> Result<(), ()> {
        if let Some(_) = self.functions.insert(identifier, symbol) {
            return Err(());
        };
        Ok(())
    }
}
