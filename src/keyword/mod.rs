use std::{error::Error, fmt};

#[derive(Debug, Clone)]
struct KeywordError;

impl fmt::Display for KeywordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

trait Keyword {
    fn run(&self, args: [String]) -> Result<bool, KeywordError>;
}

pub struct KewordRegistry {
    collection: HashMap<String, impl Keyword>
}

impl KewordRegistry {
    fn register<V: impl Keyword>(&mut self, key: String, value: &V) -> Option<V>  {
        self.collection.insert(key, value)
    }

    pub fn run_keyword(&self, key: String, args: [String]) -> Result<bool, KeywordError> {
        match self.collection.get(key) {
            None => Ok(false),
            Some(kw) => kw.run(args),
        }
    }
}

pub fn generate_registry() -> KewordRegistry {
    let kr = KewordRegistry {
        collection: HashMap::new(),
    }

    //register keywords

    kr
}