#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut i = 0 as usize;
        let mut j = nums.len() - 1;
        while i < j {
            if nums[j] == -nums[i] {
                return nums[j];
            }

            if nums[j] > -nums[i] {
                j -= 1;
            } else {
                i += 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn run() {
        let ans = Solution::find_max_k(vec![0, 0]);
        println!(" {ans} ");

        let ans = Solution::find_max_k(vec![-2, 1, 1, 1, 1, 1, 1]);
        println!(" {ans} ");

        let ans = Solution::find_max_k(vec![1, 2, 3]);
        println!(" {ans} ");

        let ans = Solution::find_max_k(vec![1, 2, -1]);
        println!(" {ans} ");
    }
}
