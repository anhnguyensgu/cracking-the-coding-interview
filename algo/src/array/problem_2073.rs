#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn time_required_to_buy(mut tickets: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        while tickets[k as usize] > 0 {
            let mut time = 0;
            for (i, cur) in tickets.iter_mut().enumerate() {
                if *cur > 0 {
                    time += 1;
                    *cur -= 1;
                }
                if i == k as usize && *cur == 0 {
                    return ans + time;
                }
            }
            ans += time;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_run() {
        let tickets = vec![2, 3, 2];
        let k = 2;
        let ans = Solution::time_required_to_buy(tickets, k);
        println!("{ans}");

        let tickets = vec![5, 1, 1, 1];
        let k = 0;
        let ans = Solution::time_required_to_buy(tickets, k);
        println!("{ans}");

        let tickets = vec![84, 49, 5, 24, 70, 77, 87, 8];
        let k = 3;
        let ans = Solution::time_required_to_buy(tickets, k);
        println!("{ans}");
    }
}
