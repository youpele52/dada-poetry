use std::env;

#[derive(Debug)]
pub struct Args {
    pub config_filename: String,
}

impl Args {
    pub fn new() -> Args {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            eprintln!("\n\nUsage:\ncargo run <config_filename> \n\n");
            std::process::exit(1);
        }

        Args {
            config_filename: args[1].to_string(),
        }
    }
}
