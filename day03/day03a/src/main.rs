extern crate ndarray;
extern crate regex;

use ndarray::Array2;
use regex::Regex;

use std::env;
use std::fs;
use std::io::{Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut board = Array2::<u32>::zeros((2000, 2000));

    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let text = fs::read_to_string(filename).expect("Something bad happened reading file");

    let mut patches = Array2::<usize>::zeros((1284, 5));

    for cap in re.captures_iter(&text) {
    	let idx = cap[1].parse::<usize>().unwrap();
    	let x = cap[2].parse::<usize>().unwrap();
    	let y = cap[3].parse::<usize>().unwrap();
    	let x_span = cap[4].parse::<usize>().unwrap();
    	let y_span = cap[5].parse::<usize>().unwrap();
    	
    	patches[[idx, 0]] = x;
    	patches[[idx, 1]] = y;
    	patches[[idx, 2]] = x_span;
    	patches[[idx, 3]] = y_span;

    	for i in x..(x+x_span) {
    		for j in y..(y+y_span) {
    			board[[i, j]] = board[[i, j]] + 1;
    		}
    	}
    }

    let mut count: u32 = 0;
    for i in 0..2000 {
    	for j in 0..2000 {
    		if board[[i, j]] > 1 {
    			count = count + 1;
    		}
    	}
    }

    println!("Covered by multiples: {}", count);

    // There IS no patch[0]
    for a in 1..1283 {
    	let x = patches[[a, 0]];
    	let y = patches[[a, 1]];
    	let xs = patches[[a, 2]];
    	let ys = patches[[a, 3]];

    	let mut counter = 0;
    	for i in x..(x+xs) {
    		for j in y..(y+ys) {
    			if board[[i, j]] > 1 {
    				counter = counter + 1;
    			}
    		}
    	}
    	if counter == 0 {
    		println!("Standalone patch: {}", a);
    	}
    }

    Ok(())
}