#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut times = vec![];
        for i in 0..position.len() {
            times.push((position[i], speed[i]));
        }
        times.sort_by(|a,b|a.0.cmp(&b.0));

        let mut stack = vec![];
        for (x2, s2) in times.iter() {
            let time = (target - x2) as f64 / *s2 as f64;
            while !stack.is_empty() {
                let first = stack.last().unwrap();
                if time < *first {
                   break;
                }
                stack.pop();
            }
        
            stack.push(time);
        }
        stack.len() as i32
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::greedy::problem_853::Solution;

    #[test]
    fn run() {
        let target = 12;
        let position = vec![10, 8, 0, 5, 3];
        let speed = vec![2, 4, 1, 1, 3];
        let ans = Solution::car_fleet(target, position, speed);
        assert_eq!(ans, 3);

        let target = 10;
        let position = vec![3];
        let speed = vec![3];
        let ans = Solution::car_fleet(target, position, speed);
        assert_eq!(ans, 1);

        let target = 100;
        let position = vec![0, 2, 4];
        let speed = vec![4, 2, 1];
        let ans = Solution::car_fleet(target, position, speed);
        assert_eq!(ans, 1);

        let target = 10;
        let position = vec![0, 4, 2];
        let speed = vec![2, 1, 3];
        let ans = Solution::car_fleet(target, position, speed);
        assert_eq!(ans, 1);

        let target = 10;
        let position = vec![8, 3, 7, 4, 6, 5];
        let speed = vec![4, 4, 4, 4, 4, 4];
        let ans = Solution::car_fleet(target, position, speed);
        assert_eq!(ans, 6);

        let target = 31;
        let position = vec![5, 26, 18, 25, 29, 21, 22, 12, 19, 6];
        let speed = vec![7, 6, 6, 4, 3, 4, 9, 7, 6, 4];
        let ans = Solution::car_fleet(target, position, speed);
        assert_eq!(ans, 6);

        let target = 13;
        let position = vec![10, 2, 5, 7, 4, 6, 11];
        let speed = vec![7, 5, 10, 5, 9, 4, 1];
        let ans = Solution::car_fleet(target, position, speed);
        assert_eq!(ans, 2);
    }
}
