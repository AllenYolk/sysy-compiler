use std::collections::HashMap;

/// A counter for Koopa named symbols.
///
/// This struct is used to generate unique names for Koopa named symbols.
/// The counter is actually a `HashMap` from `String` to `usize`,
/// where the `String` is the name of the symbol and the `usize` is the counter.
/// The default value of the counter is 0.
pub struct NamedSymbolCounter {
    counter: HashMap<String, usize>,
}

impl NamedSymbolCounter {
    /// Create a new `NamedSymbolCounter`.
    pub fn new() -> Self {
        Self {
            counter: HashMap::new(),
        }
    }

    /// Increase the counter of the symbol with the given id.
    pub fn inc(&mut self, id: &str) {
        let counter = self.counter.entry(id.to_string()).or_insert(0);
        *counter += 1;
    }

    /// Get the counter of the symbol with the given name.
    pub fn get_count(&self, id: &str) -> Option<usize> {
        self.counter.get(id).map(|&x| x)
    }

    /// Get the full name of the symbol with the given name.
    ///
    /// The full symbol name has the form "{id}_{counter}".
    pub fn get_named_symbol(&self, id: &str) -> Option<String> {
        match self.get_count(id) {
            Some(c) => Some(format!("{}_{}", id, c)),
            None => None,
        }
    }

    /// Increase the counter of the symbol with the given id, and return the full name of the symbol.
    ///
    /// The full symbol name has the form "{id}_{counter}"
    pub fn inc_and_get_named_symbol(&mut self, id: &str) -> Result<String, ()> {
        self.inc(id);
        let Some(sym) = self.get_named_symbol(id) else {
            return Err(());
        };
        Ok(sym)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nsc_test() {
        let mut nsc = NamedSymbolCounter::new();
        assert_eq!(nsc.get_count("@a"), None);
        assert_eq!(nsc.get_named_symbol("@a"), None);
        nsc.inc("@a");
        assert_eq!(nsc.get_count("@a"), Some(1));
        assert_eq!(nsc.get_named_symbol("@a"), Some("@a_1".to_string()));
        nsc.inc("@a");
        assert_eq!(nsc.get_count("@a"), Some(2));
        assert_eq!(nsc.get_named_symbol("@a"), Some("@a_2".to_string()));
        nsc.inc("%b");
        assert_eq!(nsc.get_count("%b"), Some(1));
        assert_eq!(nsc.get_named_symbol("%b"), Some("%b_1".to_string()));
        nsc.inc("@a");
        assert_eq!(nsc.get_count("@a"), Some(3));
        assert_eq!(nsc.get_named_symbol("@a"), Some("@a_3".to_string()));
    }
}
