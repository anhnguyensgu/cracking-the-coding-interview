#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        use std::collections::BTreeMap;
        let mut times = BTreeMap::new();
        for i in 0..position.len() {
            times.insert(position[i], speed[i]);
        }

        let mut stack = vec![];

        for (x2, s2) in times.iter() {
            while !stack.is_empty() {
                let &(x1, s1) = stack.last().unwrap();
                println!("{x1} {s1} vs {x2} {s2}");
                if s2 >= s1 {
                    break;
                }

                let t: i32 = (x2 - x1) / (s1 - s2);
                let meet_at = x1 + t * s1;
                let meet_at2 = x2 + t * s2;
                println!("meet at {meet_at} when {t}");
                println!("meet at {meet_at2} when {t}");
                if meet_at > target || meet_at2 > target {
                    break;
                }
                stack.pop();
            }
            stack.push((x2, s2));
            println!("{stack:?}");
        }

        println!("==========================");
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
