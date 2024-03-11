use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    nums.into_iter()
        .for_each(|num| *map.entry(num).or_insert(0) += 1);
    let mut vec: Vec<(i32, i32)> = map.into_iter().collect();
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    vec.iter().take(k as usize).map(|x| x.0).collect()
}

#[cfg(test)]
mod tests {
    use crate::medium::top_frequent_k_element_347::top_k_frequent;

    #[test]
    fn returns_correct_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let expected = vec![1, 2];
        let k = 2;

        let result = top_k_frequent(nums, k);

        assert_eq!(result, expected);
    }

    #[test]
    fn returns_correct_2() {
        let nums = vec![1];
        let expected = vec![1];
        let k = 1;

        let result = top_k_frequent(nums, k);

        assert_eq!(result, expected);
    }
}
