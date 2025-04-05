pub mod args;
pub mod config;
pub mod md_file;
pub mod cook;
pub mod utils;

pub struct ReadOutput {
    pub content: Option<String>,
    pub config: config::Config,
}

pub fn new() -> ReadOutput {
    let args_: args::Args = args::Args::new();
    let config: config::Config = config::Config::new(args_.config_filename);
    let content: Option<String> = if config.input_filename.ends_with(".md") {
        md_file::read_md_file(config.input_filename.as_str()).ok()
    } else {
        None
    };
    return ReadOutput { config, content };
}
