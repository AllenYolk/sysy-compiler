pub struct TempVariableManager<'a> {
    next: usize,
    pub ret: Option<&'a str>,
}

impl<'a> TempVariableManager<'a> {
    pub fn new() -> Self {
        Self { next: 0, ret: None }
    }

    pub fn new_temp_var(&mut self) -> String {
        let cur = self.next;
        self.next += 1;
        format!("%{}", cur)
    }
}
