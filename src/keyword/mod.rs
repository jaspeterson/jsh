use std::{error::Error, fmt, collections::HashMap};

mod set;

#[derive(Debug, Clone)]
pub struct KeywordError {
    msg: String
}

impl KeywordError {
    fn new(m: &str) -> KeywordError {
        KeywordError{
            msg: m.to_string(),
        }
    }
}

impl fmt::Display for KeywordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for KeywordError {
    fn description(&self) -> &str {
        &self.msg
    }
}

pub trait Keyword {
    fn run(&self, args: &[String]) -> Result<bool, KeywordError>;
}

pub struct KewordRegistry<T> {
    collection: HashMap<String, T>
}

impl<K: Keyword> KewordRegistry<K> {
    fn register(&mut self, key: String, value: K) -> Option<K>  {
        self.collection.insert(key, value)
    }

    pub fn run_keyword(&self, key: String, args: &[String]) -> Result<bool, KeywordError> {
        match self.collection.get(&key) {
            None => Ok(false),
            Some(kw) => kw.run(args),
        }
    }
}

pub fn generate_registry() -> KewordRegistry<impl Keyword> {
    let mut kr = KewordRegistry {
        collection: HashMap::new(),
    };

    //register keywords
    kr.register(String::from("set"), set::Set{});

    kr
}