mod cli;
mod reader;
mod converter;
mod writer;
mod errors;
mod utils;


use cli::get_args;
use reader::read_json;
use converter::convert;
use writer::write_csv;


fn main() -> anyhow::Result<()> {
    let args = get_args();

    let json = read_json(&args.input)?;
    let (headers, rows) = convert(json)?;
    write_csv(&args.output, headers, rows)?;

    println!("✅ Conversion completed!");
    Ok(())
}
