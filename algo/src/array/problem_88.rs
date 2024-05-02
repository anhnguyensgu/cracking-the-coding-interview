#[allow(dead_code)]
struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = 0;
        let mut j = 0;
        while i < m && j < n {}
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn run() {
        let mut num1 = vec![];
        let mut num2 = vec![];
        let m = 0;
        let n = 1;
        Solution::merge(&mut num1, m, &mut num2, n);
    }
}
