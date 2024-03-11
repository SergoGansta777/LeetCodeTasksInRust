use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut viewed_nums: HashSet<i32> = HashSet::with_capacity(nums.len());
    for &num in nums.iter() {
        if !viewed_nums.insert(num) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::contains_duplicate;

    #[test]
    fn returns_correct_answer_1() {
        let nums = vec![1, 2, 3, 1];
        let expected = true;

        let result = contains_duplicate(nums);

        assert_eq!(result, expected)
    }

    #[test]
    fn returns_correct_anser_2() {
        let nums = vec![1, 2, 3, 4];
        let expected = false;

        let result = contains_duplicate(nums);

        assert_eq!(result, expected)
    }

    #[test]
    fn returns_correct_anser_3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let expected = true;

        let result = contains_duplicate(nums);

        assert_eq!(result, expected)
    }
}
