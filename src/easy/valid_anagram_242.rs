use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut difference: HashMap<char, i64> = HashMap::new();

    for (char_s, char_t) in s.chars().zip(t.chars()) {
        *difference.entry(char_s).or_default() += 1;
        *difference.entry(char_t).or_default() -= 1;
    }

    difference.into_values().all(|difference| difference == 0)
}

#[cfg(test)]
mod tests {
    use super::is_anagram;

    #[test]
    fn handle_correct_anagrams() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        let expected = true;

        let result = is_anagram(s, t);

        assert_eq!(result, expected)
    }

    #[test]
    fn handle_incorrect_anagrams() {
        let s = "rat".to_string();
        let t = "car".to_string();
        let expected = false;

        let result = is_anagram(s, t);

        assert_eq!(result, expected)
    }
}
