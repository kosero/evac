use std::collections::HashMap;

pub struct Environment {
    pub variables: HashMap<String, i64>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<i64> {
        self.variables.get(name).cloned()
    }

    pub fn set(&mut self, name: String, value: i64) {
        self.variables.insert(name, value);
    }
}

