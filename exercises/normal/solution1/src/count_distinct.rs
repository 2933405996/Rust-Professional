use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let split_str: Vec<&str> = input_str.split(',').collect();
    let mut distinct_chars: HashSet<&str> = HashSet::new();
    for c in split_str {
        distinct_chars.insert(c);
    }
    distinct_chars.len()
}
