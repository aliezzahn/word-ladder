use std::collections::{HashSet, VecDeque};

pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let word_set: HashSet<String> = word_list.into_iter().collect();
    
    // If the end_word is not in the word_list, return 0
    if !word_set.contains(&end_word) {
        return 0;
    }
    
    let mut queue = VecDeque::new();
    queue.push_back((begin_word.clone(), 1));
    
    // Remove the begin_word from the set to avoid revisiting
    let mut word_set = word_set;
    word_set.remove(&begin_word);
    
    while let Some((current_word, steps)) = queue.pop_front() {
        // If we reach the end_word, return the number of steps
        if current_word == end_word {
            return steps;
        }
        
        // Generate all possible one-letter transformations
        for i in 0..current_word.len() {
            let mut chars: Vec<char> = current_word.chars().collect();
            for c in 'a'..='z' {
                chars[i] = c;
                let new_word: String = chars.iter().collect();
                
                // If the new_word is in the set, add it to the queue
                if word_set.contains(&new_word) {
                    queue.push_back((new_word.clone(), steps + 1));
                    word_set.remove(&new_word); // Avoid revisiting
                }
            }
        }
    }
    
    // If we exhaust the queue without finding the end_word, return 0
    0
}