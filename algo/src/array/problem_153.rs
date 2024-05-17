#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut j = 0;
        let mut i = 1;
        let mut cur = 1;
        while i < ratings.len() {
            if ratings[i] == ratings[i - 1] {
                let z = i - 1;
                (0..=z - j).for_each(|c| {
                    ans += cur - c;
                });
                println!("{j} {z} count {ans} eq");
                cur = 1;
                j = i;
            } else if ratings[i] > ratings[i - 1] {
                let z = i - 1;
                (0..=z - j).for_each(|c| {
                    ans += cur + c;
                });
                println!("{j} {z} count {ans} greater");
                j = i;
                cur = cur - z + j;
            }
            i += 1;
        }

        if j < ratings.len() {
            let len = ratings.len();
            println!("{j} {len} cur: {cur}");
            (0..ratings.len() - j).for_each(|c| {
                ans += cur + c;
            });
        }

        ans as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn run() {
        let ratings = vec![1, 0, 2];
        let ans = Solution::candy(ratings);
        let expected = 5;
        assert_eq!(ans, expected);
        println!("==============");

        let ratings = vec![3, 2, 1];
        let ans = Solution::candy(ratings);
        let expected = 6;
        assert_eq!(ans, expected);

        println!("==============");
        let ratings = vec![1, 2, 2];
        let ans = Solution::candy(ratings);
        let expected = 4;
        assert_eq!(ans, expected);

        println!("==============");
        let ratings = vec![1, 2, 3];
        let ans = Solution::candy(ratings);
        let expected = 6;
        assert_eq!(ans, expected);

        println!("==============");
        // 0 0 count 1 greater
        // 1 2 count 6 eq
        // 3 5 cur: 1
        let ratings = vec![1, 3, 2, 2, 1];
        let ans = Solution::candy(ratings);
        let expected = 7;
        assert_eq!(ans, expected);
    }
}
