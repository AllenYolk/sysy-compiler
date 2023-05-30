use std::collections::HashMap;

pub struct Scopes<'a> {
    functions: HashMap<&'a str, &'a str>,
    values: Vec<HashMap<&'a str, &'a str> >,
}

impl<'a> Scopes<'a> {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            values: vec![HashMap::new()],
        }
    }

    pub fn enter(&mut self) {
        self.values.push(HashMap::new());
    }

    pub fn exit(&mut self) {
        self.values.pop();
    }
}