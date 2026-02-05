use clap::Parser;

#[derive(Parser)]

pub struct Args {
    pub file: String,

    #[arg(long)]
    pub level: Option<String>,

    #[arg(long)]
    pub since: Option<String>,

    #[arg(long)]
    pub out: Option<String>,

}

pub fn get_args() -> Args {
    Args::parse()
}