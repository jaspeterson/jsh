pub mod tokenizer;

use std::io::{self, BufRead, Write};

pub fn receive_user_input() {
    loop {
        //get a line of input
        let input = prompt_input();
        match input {
            Ok(cmd) => {
                println!("got a line: {}", cmd);
                let cmd = String::from(cmd.trim());
                let i = tokenizer::Tokenizer::tokenize(&cmd);

                println!("{:?}", i.tokens);
            },
            Err(error) => println!("got an error: {}", error),
        }
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