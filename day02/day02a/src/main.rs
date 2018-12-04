use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let file = File::open(filename)?;

   	let mut twos = 0;
   	let mut threes = 0;

    for line in BufReader::new(file).lines() {
    	let mut counts: HashMap<char, i8> = HashMap::new();

    	for c in line?.chars() {
			//print!("{}", c);
			let count = counts.entry(c).or_insert(0);
			*count += 1;
    	}

    	let mut has_two = false;
    	let mut has_three = false;
    	for (_k, v) in &counts {
    		if *v == 2 {
    			has_two = true;
    		} else if *v == 3 {
    			has_three = true;
    		}
    	}

    	if has_two == true {
    		twos = twos + 1;
    	}

    	if has_three {
    		threes = threes + 1;
    	}
    }

    println!("Checksum: {}", twos * threes);

    Ok(())    
}
