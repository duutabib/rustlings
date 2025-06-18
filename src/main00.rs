// checking for duplicates when input is really large or streamed
// using bloom filter data structure 
// bloom filter is a space efficient data structure 
// for representing a set 
// because hashser there collions, so we need to set a tolerance for 
// false positive rate

use bloom::{ASMS, BloomFilter};


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
            let result = contains_duplicate_large(&nums);
            println!("Results: {}", result);
        }
        Err(e) => {
            eprintln!("Error: Please provide valid integers as arguments {}", e);
        }
    }
}

fn contains_duplicate_large(nums: &[i32]) ->  bool {
    // expected num of items..
    let expected_num_items = 1_000_000;
    let tolerance = 0.01;

    // Create a HashSet to store unique numbers
    let mut filter = BloomFilter::with_rate(tolerance, expected_num_items);
   
    // Add to filter or return true 
    // if the filter contains integer
    // else false
    for &num in nums {
        if filter.contains(&num) {
            return true;
    }
    filter.insert(&num);
    }
    false
}

