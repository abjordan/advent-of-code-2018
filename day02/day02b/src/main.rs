extern crate edit_distance;

use edit_distance::edit_distance;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename)?;

    let mut entries: Vec<String> = Vec::new();

    for line in BufReader::new(file).lines() {
    	entries.push(line?);
    }

    // This is n^2, which kinda sucks. I'm sure there's a more
    // efficient algorithm for this, since what we're doing is
    // a matrix Levenshtein distance calculation...
    let mut found = false;
    for e in &entries {
    	for f in &entries {
    		if edit_distance(e, f) == 1 {
    			let mut same = String::new();
    			for i in 0..e.len() {
    				if e.chars().nth(i) == f.chars().nth(i) {
    					same.push(e.chars().nth(i).expect("WHAA???"));
    				}
    			}
    			println!("Same: {}", same);
    			found = true;
    		}
    	}

    	if found == true {
    		break;
    	}
    }

    Ok(())
}