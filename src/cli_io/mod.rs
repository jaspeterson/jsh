pub mod tokenizer;

use std::io::{self, BufRead, Write};

pub fn receive_user_input() -> Result<Vec<String>, io::Error> {
    let input = prompt_input();
    match input {
        Ok(cmd) => {
            let cmd = String::from(cmd.trim());
            let tokens = tokenizer::tokenize(&cmd);
            Ok(tokens)
        },
        Err(error) => {
            println!("cannot read user input");
            Err(error)
        },
    }
}

//Todo: make prompt configurable by shell environment variable
//Todo: make environment variables lol
fn prompt_input() -> Result<String, io::Error> {
    print!("jsh --> "); // print the configured prompt
    io::stdout().flush()?;
    let s = read_line()?;
    Ok(s)
}

fn read_line() -> Result<String, io::Error> {
    let mut buff = String::new();
    let input = io::stdin().lock().read_line(&mut buff);
    match input {
        Ok(_size) => Ok(buff),
        Err(e) => Err(e),
    }
}