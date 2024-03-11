use std::collections::BTreeSet;

pub fn third_max(nums: Vec<i32>) -> i32 {
    let top: Vec<i32> = nums
        .into_iter()
        .collect::<BTreeSet<i32>>()
        .into_iter()
        .rev()
        .take(3)
        .collect();

    match top.len() {
        3 => *top.last().unwrap(),
        _ => *top.first().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::third_max_414::third_max;

    #[test]
    fn returns_correct() {
        let nums = vec![3, 2, 1];
        let expected = 1;

        let result = third_max(nums);

        assert_eq!(result, expected);
    }
    #[test]
    fn returns_max_if_third_max_not_exist() {
        let nums = vec![1, 2];
        let expected = 2;

        let result = third_max(nums);

        assert_eq!(result, expected);
    }

    #[test]
    fn returns_correct_if_dublicates_exist() {
        let nums = vec![2, 2, 3, 1];
        let expected = 1;

        let result = third_max(nums);

        assert_eq!(result, expected);
    }
}
