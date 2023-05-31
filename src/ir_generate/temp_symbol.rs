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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_temp_symbol_test() {
        let mut tsm = TempSymbolManager::new();
        let v1 = tsm.new_temp_symbol();
        let v2 = tsm.new_temp_symbol();
        assert_eq!(v1, "%0");
        assert_eq!(v2, "%1");
    }
}
