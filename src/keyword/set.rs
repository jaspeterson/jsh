use crate::keyword;

// Implements the command 'set'
// Maps an environment variable
// Requires 2 arguments, key and value
pub struct Set;

impl keyword::Keyword for Set {
    fn run(&self, args: &[String]) -> Result<bool, keyword::KeywordError> {
        if args.len() != 2 {
           return Err(keyword::KeywordError::new("expected 2 arguments"))
        }

        println!("setting environment variable {}={}", args[0], args[1]);
        Ok(true)
    }
}