use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let file = File::open(filename)?;

    let mut total = 0;

    for line in BufReader::new(file).lines() {
		let line = line.expect("Unable to read line");
		let parsed: i32 = line.trim().parse().expect("Unable to parse value");
		total = total + parsed;
    }

    println!("Total: {}", total);

    Ok(())
}