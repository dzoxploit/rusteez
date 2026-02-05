mod cli;
mod parser;
mod filter;
mod output;

use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let args = cli::get_args();

    let file = File::open(&args.file)?;
    let reader = BufReader::new(file);

    let logs: Vec<_> = reader.lines()
        .filter_map(|l| parser::parse_line(&l.ok()?))
        .collect();

    let filtered = filter::filter_logs(logs, &args.level, &args.since);

    if let Some(out) = args.out {
        output::write_csv(&out, &filtered)?;
        println!("Saved to {}", out);
    } else {
        for log in filtered {
            println!("{} {} {}", log.timestamp, log.level, log.message);
        }
    }

    Ok(())
}
