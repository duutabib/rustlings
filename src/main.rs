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
    if nums.is_empty() {
        return false;
    }
    
    // Create a HashSet to store unique numbers
    let unique: HashSet<_> = nums.iter().collect();
   
    // If the length of the HashSet is less than the 
    // length of the input vector, 
    // there are duplicates
    if nums.len() > unique.len() {
        return true
    }
    false
}

