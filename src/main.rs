mod cli_io;
mod environment;
mod keyword;

use std::env;

struct Args {
    arg1: Option<String>,
    help: bool,
}

impl Args {
    fn new() -> Args {
        Args {
            arg1: None,
            help: false,
        }
    }
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    // parse args
    let args = parse_args(args);
    let args = match args {
        Ok(a) => a,
        Err(error) => {
            print_help();
            panic!("{}", error);
        },
    };

    if args.help {
        print_help();
        return
    }

    //create environment
    let mut env = environment::Environment::new();
    let kr = keyword::generate_registry();

    loop {
        let input = cli_io::receive_user_input();
        match input {
            Ok(tokens) => {
                if let Some(v) = tokens.get(0) {
                    match kr.run_keyword(v.to_string(), &tokens[1..]) {
                        Err(e) => println!("{}", e),
                        _ => (),
                    }
                }
            },
            Err(_error) => continue,
        }
    }
}

fn print_help() {
    println!("jsh --- the joe shell\n\nUSAGE:\n\tjsh [FLAGS]\n\nFLAGS:\n\t-f, --flag\tDescription of flag\n\t-h, --help\tPrint help menu");
}

fn parse_args(args: Vec<String>) -> Result<Args, String> {
    let mut a = Args::new();

    let mut arg_iter = args.iter();
    arg_iter.next(); // skip the first arg
    for (pos, arg) in args.iter().enumerate() {
        if pos == 0 {continue;}
        match arg.as_str() {
            "" => a.arg1 = Some(String::from("")),
            "--help" => a.help = true,
            "-h" => a.help = true,
            _ => {
                return Err(String::from(format!("unknown argument: {}", arg)));
            },
        }
    }

    Ok(a)
}
