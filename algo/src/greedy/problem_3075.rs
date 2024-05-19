#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort_unstable();
        let mut sum = 0i64;
        let mut count = 0;
        (happiness.len() - k as usize..happiness.len())
            .rev()
            .for_each(|i| {
                let decrease = happiness[i] as i64 - count;
                if decrease > 0 {
                    sum += decrease;
                }
                count += 1;
            });
        sum
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn run() {
        let happiness = vec![1, 2, 3];
        let k = 2;
        let ans = Solution::maximum_happiness_sum(happiness, k);
        assert_eq!(ans, 4);

        let happiness = vec![1, 1, 1, 3];
        let k = 3;
        let ans = Solution::maximum_happiness_sum(happiness, k);
        assert_eq!(ans, 3);

        let happiness = vec![2, 3, 4, 5];
        let k = 1;
        let ans = Solution::maximum_happiness_sum(happiness, k);
        assert_eq!(ans, 5);
    }
}
