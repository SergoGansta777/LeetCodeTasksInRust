use std::collections::HashMap;
use std::collections::HashSet;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut have_at = HashMap::with_capacity(nums.len());
    for (i, num) in nums.into_iter().enumerate() {
        let needed = target - num;
        if let Some(&j) = have_at.get(&needed) {
            return vec![i as i32, j as i32];
        }
        have_at.insert(num, i);
    }
    unreachable!()
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut viewed_nums: HashSet<i32> = HashSet::with_capacity(nums.len());
    for &num in nums.iter() {
        if !viewed_nums.insert(num) {
            return true;
        }
    }
    false
}

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
