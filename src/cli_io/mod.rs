pub mod interpreter;

use std::io::{self, BufRead};

pub fn read_line() -> Result<String, io::Error> {
    let mut buff = String::new();
    let input = io::stdin().lock().read_line(&mut buff);
    match input {
        Ok(_size) => Ok(buff),
        Err(e) => Err(e),
    }
}