pub struct TempSymbolManager {
    next: usize,
}

impl TempSymbolManager {
    pub fn new() -> Self {
        Self { next: 0 }
    }

    pub fn new_temp_symbol(&mut self) -> String {
        let cur = self.next;
        self.next += 1;
        format!("%{}", cur)
    }
}
