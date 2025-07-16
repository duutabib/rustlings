// checking for duplicates by comparing lenghts
use std::env;
use bloom::BloomFilter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <str1> <str2>", args[0]);
        return;
    }

    let word = &args[1];
    let word0 = &args[2];
    
    let result = is_large_anagram(word, word0);
    println!("Are '{}' and '{}' anagrams? {}", word, word0, result);
}

fn is_large_anagram<R1: Read, R2: Read>(stream1: R1, mut stream2: R2) -> Result<bool> {
    if stream1.is_empty() || stream2.is_empty() {
        return false;
    }

    let mut freqs = HashMap::new();
    let mut stream1_len  = 0;
    let mut stream2_len = 0;
    
    // build frequency map from first stream
    for byte in stream1.bytes() {
        let c = byte? as char;

        // Since we only care about alphabetic
        if c.is_alphabetic() {
            let lower_c = c.to_ascii_lowercase();
            *freqs.entry(lower_c).or_insert(0i64) += 1;
            stream1_len += 1;
        }
    }
    
    // build frequency map from second stream
    for byte in stream2.bytes() {
        let c = byte? as char;

        // Since we only care about alphabetic
        if c.is_alphabetic() {
            let lower_c = c.to_ascii_lowercase();
            *freqs.entry(lower_c).or_insert(0) -= 1;
            stream2_len += 1;
        }
    }
    
    // check if the frequency map is empty
    if stream1_len != stream2_len {
        return Ok(false);
    }


    // if they are anagrams, all chars counts are equal
    Ok(freqs.values().all(|&v| v == 0))
}
