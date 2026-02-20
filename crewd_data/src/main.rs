use anyhow::{Context, Result};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
	anyhow::bail!("Usage: {} <csv-file>", args[0]);
    }
    let filename = args[1].clone();
    println!("getting data from '{filename}'");
    let schedule = lib::read_schedule(filename)?;
    for event in schedule {
	println!("{:#?}", event);
    }
    Ok(())
}
