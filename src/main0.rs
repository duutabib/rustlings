// Checking for duplicates by tracking unique elements in set.
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
            let result = has_duplicates(nums);
            println!("Results: {}", result);
        }
        Err(e) => {
            eprintln!("Error: Please provide valid integers as arguments {}", e);
        }
    }
}

fn has_duplicates(nums: Vec<i32>) -> bool {
    if nums.is_empty() {
        return false;
    }

    // Create a HashSet to store unique numbers
    let mut seen = HashSet::new();
    for num in nums {
        // Check if the number is already in the set
        if seen.contains(&num) {
            return true;
        }
        seen.insert(num);
    }
    false
}
