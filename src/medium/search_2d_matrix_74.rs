use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len() as i32;
        let cols = matrix[0].len() as i32;

        let mut first = 0;
        let mut last = rows * cols - 1;
        while first <= last {
            let mid = first + (last - first) / 2;
            let mid_value = matrix[(mid / cols) as usize][(mid % cols) as usize];

            match mid_value.cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Less => first = mid + 1,
                Ordering::Greater => last = mid - 1,
            }
        }

        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_handles_true_answer() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;
        let expected = true;

        let result = Solution::search_matrix(matrix, target);

        assert_eq!(result, expected);
    }

    #[test]
    fn correctly_handles_false_answer() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 13;
        let expected = false;

        let result = Solution::search_matrix(matrix, target);

        assert_eq!(result, expected);
    }

    #[test]
    fn correctly_handles_edges() {
        let matrix = vec![vec![1, 1]];
        let target = 2;
        let expected = false;

        let result = Solution::search_matrix(matrix, target);

        assert_eq!(result, expected);
    }
}
