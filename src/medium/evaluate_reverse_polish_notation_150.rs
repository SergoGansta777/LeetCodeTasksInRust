impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for token in tokens {
            match &token[..] {
                "+" => {
                    let second_operand = stack.pop().unwrap();
                    let first_operand = stack.pop().unwrap();
                    stack.push(first_operand + second_operand)
                }
                "-" => {
                    let second_operand = stack.pop().unwrap();
                    let first_operand = stack.pop().unwrap();
                    stack.push(first_operand - second_operand)
                }
                "*" => {
                    let second_operand = stack.pop().unwrap();
                    let first_operand = stack.pop().unwrap();
                    stack.push(first_operand * second_operand)
                }
                "/" => {
                    let second_operand = stack.pop().unwrap();
                    let first_operand = stack.pop().unwrap();
                    stack.push(first_operand / second_operand)
                }
                value => stack.push(value.parse::<i32>().unwrap()),
            }
        }

        stack[0]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_corect_answer_1() {
        let tokens = vec!["2", "1", "+", "3", "*"];
        let expected = 9;

        let result = Solution::eval_rpn(tokens.into_iter().map(|x| x.to_string()).collect());

        assert_eq!(result, expected);
    }

    #[test]
    fn return_corect_answer_2() {
        let tokens = vec!["4", "13", "5", "/", "+"];
        let expected = 6;

        let result = Solution::eval_rpn(tokens.into_iter().map(|x| x.to_string()).collect());

        assert_eq!(result, expected);
    }

    #[test]
    fn return_corect_answer_3() {
        let tokens = vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ];
        let expected = 22;

        let result = Solution::eval_rpn(tokens.into_iter().map(|x| x.to_string()).collect());

        assert_eq!(result, expected);
    }
}
