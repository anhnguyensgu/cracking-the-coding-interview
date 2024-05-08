#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn car_fleet(target: i32, mut position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut time = 0;
        for i in 0..position.len() {
            time = time.max((target - position[i]) / speed[i])
        }
        for _ in 0..time as usize {
            for i in 0..position.len() {
                position[i] += speed[i];
            }
        }
        ans
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
        println!("{}", Solution::car_fleet(target, position, speed));

        let target = 10;
        let position = vec![0, 2, 4];
        let speed = vec![4, 2, 1];
        println!("{}", Solution::car_fleet(target, position, speed));
    }
}
