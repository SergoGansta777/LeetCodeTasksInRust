impl Solution {
    fn is_speed_enough(piles: &Vec<i32>, h: i32, speed: i32) -> bool {
        let mut needed_time = 0;
        for pile in piles {
            needed_time += pile / speed;
            if pile % speed != 0 {
                needed_time += 1
            }
        }

        needed_time <= h
    }

    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut min_speed = 1i32;
        let mut max_speed = i32::MAX;

        while min_speed < max_speed {
            let mid_speed = min_speed + (max_speed - min_speed) / 2;
            match Self::is_speed_enough(&piles, h, mid_speed) {
                false => min_speed = mid_speed + 1,
                true => max_speed = mid_speed,
            }
        }

        min_speed
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_handles_1() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        let expected = 4;

        let result = Solution::min_eating_speed(piles, h);

        assert_eq!(result, expected);
    }

    #[test]
    fn correctly_handles_2() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        let expected = 30;

        let result = Solution::min_eating_speed(piles, h);

        assert_eq!(result, expected);
    }

    #[test]
    fn correctly_handles_3() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;
        let expected = 23;

        let result = Solution::min_eating_speed(piles, h);

        assert_eq!(result, expected);
    }
}
