// checking for duplicates when input is really large or streamed
// using bloom filter data structure 
// we use three hash functions to reduce collisions

use std::env;
use bloom::{ASMS, BloomFilter};
use std::hash::{hash, Hasher};  // declare two hash functions
use std::collections::hash_map::DefaultHasher; 


// optimal bits per item for ~1% false positive rate
const BITS_PER_ITEM: usize = 10;
const EXPECTED_ITEMS: usize = 1_000_000;
const FALSE_POSITIVE_RATE: f64 = 0.01;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        // print program name arg[0] and cli args arg[1..]
        println!("Usage: {} <num1> <num2> ... <numN>", args[0]);
        std::process::exit(1);
    }

    let numbers: Result<Vec<i32>, _> = args[1..]
        .iter()
        .map(|s| s.parse())
        .collect();


    match numbers {
        Ok(nums) => {
            let result = contains_duplicate_large(nums);
            println!("Results: {}", result);
        }
        Err(e) => {
            eprintln!("Error: Please provide valid integers as arguments {}", e);
            std::process::exit(1);
        }
    }
}

// setup hash functions
fn basic_hash<T: Hash>(item: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}

/// Add salt to hash function for better distribution
fn salted_hash<T: Hash>(item: &T, salt: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    salt.hash(&mut hasher);
    item.hash(&mut hasher);
    hasher.finish()
}

/// Creates a new Bloom filter with optimal params
fn creat_bloom_filter() -> impl ASMS {
    BloomFilter::with_rate(
        EXPECTED_ITEMS * BITS_PER_ITEM,
        vec![
            basic_hash,
             |x| salted_hash(x, "salt1"),
             |x| salted_hash(x, "salt2"),
        ]
    )
}


// checks for duplicates in seq of nums
pub fn contains_duplicate_large(nums: &[i32]) ->  bool {
    if nums.is_empty() {
        return false;
    }

    let mut filter = creat_bloom_filter();
    
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
    
    
    

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_contains_duplicate_large_no_duplicates() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(contains_duplicate_large(&nums), false);
    }

    #[test]
    fn test_contains_duplicate_large_with_duplicates() {
        let nums = vec![1, 2, 3, 4, 5, 1];
        assert_eq!(contains_duplicate_large(&nums), true);
    }
    
    #[test]
    fn test_contains_duplicate_large_empty() {
        let nums = vec![];
        assert_eq!(contains_duplicate_large(&nums), false);
    }

    #[test]
    fn test_contains_duplicate_large_single() {
        let nums = vec![1];
        assert_eq!(contains_duplicate_large(&nums), false);
    }
}
