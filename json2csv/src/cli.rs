use clap::Parser;

pub struct Args {
    pub input: String,
    pub output: String,
}

pub fun get_args() -> Args {
    Args::parse()
}