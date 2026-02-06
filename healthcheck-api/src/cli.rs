use clap::Parser;
use std::time::Duration; 


#[derive(Parser)]

pub struct Args {
    pub file: String,


    #[arg(long, default_value_t = 3)]
    pub retries: u32,

    #[arg(long, value_parser = humantime::parse_duration, default_value = "5s")]
    pub timeout: Duration,


    #[arg(long, default_value_t = 9898)]
    pub metrics_port: u16,
    
}