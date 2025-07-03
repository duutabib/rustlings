// checking for duplicates by comparing lenghts
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <str1> <str2>", args[0]);
        return;
    }

    let word = &args[1];
    let word0 = &args[2];
    
    let result = is_anagram(word, word0);
    println!("Are '{}' and '{}' anagrams? {}", word, word0, result);
}

fn is_anagram(word: &str, word0: &str) -> bool {
    if word.is_empty() || word0.is_empty() {
        return false;
    }
    if word.len() != word0.len() {
        return false;
    }
    
    // convert words to lowercase and sort them
    let sorted_word = sort_string(&word.to_lowercase());
    let sorted_word0 = sort_string(&word0.to_lowercase());
    
    // compare the sorted words
    sorted_word == sorted_word0
}

fn sort_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}