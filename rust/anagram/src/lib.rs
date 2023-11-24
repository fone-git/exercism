use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let lower_word = word.to_lowercase();
    let key_word = key_from_str(&lower_word);
    for &possible_anagram in possible_anagrams {
        let lower_possible_anagram = possible_anagram.to_lowercase();
        if lower_possible_anagram != lower_word && key_word == key_from_str(&lower_possible_anagram)
        {
            result.insert(possible_anagram);
        }
    }
    result
}

fn key_from_str(s: &str) -> Vec<char> {
    let mut result: Vec<char> = s.chars().collect();
    result.sort_unstable();
    result
}
