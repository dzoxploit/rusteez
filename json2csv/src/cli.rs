use clap::Parser;

#[derive(Parser)]
pub struct Args {
    pub input: String,
    pub output: String,
}

pub fn get_args() -> Args {
    Args::parse()
}