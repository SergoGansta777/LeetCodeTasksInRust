// impl Solution {
//     pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
//         let mut result: Vec<i32> = vec![0; temperatures.len()];
//         let mut last_greater_temp: Vec<(i32, usize)> = vec![]; // (temp, index)

//         for (i, val) in temperatures.iter().enumerate() {
//             while !last_greater_temp.is_empty() && *val > last_greater_temp.last().unwrap().0 {
//                 let (_, stack_index) = last_greater_temp.pop().unwrap();
//                 result[stack_index] = (i - stack_index) as i32;
//             }

//             last_greater_temp.push((*val, i));
//         }

//         result
//     }
// }

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0i32; temperatures.len()];
        let mut stack: Vec<usize> = vec![];
        for i in 0..temperatures.len() {
            // while top of stack is less than curr value, pop and update answer
            while let Some(&last) = stack.last() {
                if temperatures[last] >= temperatures[i] {
                    break;
                }
                stack.pop();
                result[last] = (i - last) as i32;
            }

            stack.push(i);
        }

        result
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_correct_answer_1() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let expected = vec![1, 1, 4, 2, 1, 1, 0, 0];

        let result = Solution::daily_temperatures(temperatures);

        assert_eq!(expected.len(), result.len());
        assert!(result == expected);
    }

    #[test]
    fn returns_correct_answer_2() {
        let temperatures = vec![30, 40, 50, 60];
        let expected = vec![1, 1, 1, 0];

        let result = Solution::daily_temperatures(temperatures);

        assert_eq!(expected.len(), result.len());
        assert!(result == expected)
    }

    #[test]
    fn returns_correct_answer_3() {
        let temperatures = vec![30, 60, 90];
        let expected = vec![1, 1, 0];

        let result = Solution::daily_temperatures(temperatures);

        assert_eq!(expected.len(), result.len());
        assert!(result == expected)
    }
}
