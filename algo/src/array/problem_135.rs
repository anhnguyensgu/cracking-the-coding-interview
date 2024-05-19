#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut ans = 1;
        let mut j = 0;
        let mut i = 1;
        let mut cur = 1;
        let mut peak = 1;
        while i < ratings.len() {
            match ratings[i].cmp(&ratings[i - 1]) {
                std::cmp::Ordering::Less => {
                    let height = i - j;
                    if height >= peak {
                        ans += height + 1;
                    } else {
                        ans += height;
                    }
                    cur = 1;
                }
                std::cmp::Ordering::Equal => {
                    cur = 1;
                    peak = cur;
                    ans += cur;
                    j = i;
                }
                std::cmp::Ordering::Greater => {
                    cur += 1;
                    peak = cur;
                    ans += cur;
                    j = i;
                }
            };
            i += 1;
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
        let ratings = vec![1, 3, 2, 2, 1];
        let ans = Solution::candy(ratings);
        let expected = 7;
        assert_eq!(ans, expected);

        println!("==============");
        let ratings = vec![1, 2, 4, 3, 2, 1];
        let ans = Solution::candy(ratings);
        let expected = 13;
        assert_eq!(ans, expected);

        println!("==============");
        let ratings = vec![1, 2, 3, 4, 3];
        let ans = Solution::candy(ratings);
        let expected = 11;
        assert_eq!(ans, expected);
    }
}
