#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_price = prices[0];
        for i in 0..prices.len() {
            if prices[i] > min_price {
                max_profit += prices[i] - min_price;
            }
            min_price = prices[i];
        }
        max_profit
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn run() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let ans = Solution::max_profit(prices);
        println!("{ans}");

        let prices = vec![1, 2, 3, 4, 5];
        let ans = Solution::max_profit(prices);
        println!("{ans}");

        let prices = vec![3, 2, 1];
        let ans = Solution::max_profit(prices);
        println!("{ans}");
    }
}
