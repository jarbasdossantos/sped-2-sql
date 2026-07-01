use std::collections::HashMap;
use std::sync::Arc;

use crate::domain::sped::SpedRecordFactory;

pub struct SpedRegistry {
    factories: HashMap<String, Arc<dyn SpedRecordFactory>>,
}

impl SpedRegistry {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn register(&mut self, factory: Arc<dyn SpedRecordFactory>) {
        self.factories.insert(factory.handle_reg().to_string(), factory);
    }

    pub fn get(&self, reg: &str) -> Option<&Arc<dyn SpedRecordFactory>> {
        self.factories.get(reg)
    }

    pub fn get_all(&self) -> &HashMap<String, Arc<dyn SpedRecordFactory>> {
        &self.factories
    }
}

impl Default for SpedRegistry {
    fn default() -> Self {
        Self::new()
    }
}
