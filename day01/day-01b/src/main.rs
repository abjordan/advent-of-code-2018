use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let mut total: i64 = 0;
    let mut values: Vec<i64> = Vec::new();
    let mut found = 0;

    values.push(0);

    loop {
    	let file = File::open(filename)?;
	    for line in BufReader::new(file).lines() {
			let line = line.expect("Unable to read line");
			let parsed: i64 = line.trim().parse().expect("Unable to parse value");
			
			total = total + parsed;
			println!("Current total: {}", total);
			for i in &values {
				if *i == total {
					println!("First repeated value: {}", total);
					found = 1;
					break;
				}
			}
			if found == 1 {
				break;
			}

			values.push(total);
	    }

	    if found == 1 {
	    	break;
	    }
	}

    Ok(())
}