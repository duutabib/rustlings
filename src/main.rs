// checking for duplicates by comparing lenghts

use std::collections::HashSet;
use std::env;


fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        // print program name arg[0] and cli args arg[1..]
        println!("Usage: {} <num1> <num2> ... <numN>", args[0]);
        return;
    }

    let numbers: Result<Vec<i32>, _> = args[1..]
        .iter()
        .map(|s| s.parse())
        .collect();


    match numbers {
        Ok(nums) => {
            let result = contains_duplicate(nums);
            println!("Results: {}", result);
        }
        Err(e) => {
            eprintln!("Error: Please provide valid integers as arguments {}", e);
        }
    }
}

fn contains_duplicate(nums: Vec<i32>) ->  bool {

    let unique: HashSet<_> = nums.iter().collect();
    
    if nums.len() > unique.len() {
        return true
    }
    false
}

