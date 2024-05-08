#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut fartherest = nums[0];
        let des = nums.len() - 1;
        let des = des as i32;
        let mut i = 1;
        while fartherest < des && i < nums.len() - 1 {
            let cur = i as i32 + nums[i];
            if i as i32 > fartherest {
                break;
            }
            if fartherest < cur {
                fartherest = cur;
            }
            i += 1;
        }
        fartherest >= des
    }
}

#[cfg(test)]
mod test {
    use crate::greedy::problem_55::Solution;

    #[test]
    fn run() {
        let nums = vec![2, 3, 1, 1, 4];
        println!("{}", Solution::can_jump(nums));

        let nums = vec![3, 2, 1, 0, 4];
        println!("{}", Solution::can_jump(nums));

        let nums = vec![0];
        println!("{}", Solution::can_jump(nums));

        let nums = vec![1, 0, 4];
        println!("{}", Solution::can_jump(nums));

        let nums = vec![1, 0, 1, 0];
        println!("{}", Solution::can_jump(nums));

        let nums = vec![1, 1, 1, 0];
        println!("{}", Solution::can_jump(nums));

        let nums = vec![0, 1, 1, 0];
        println!("{}", Solution::can_jump(nums));

        let nums = vec![2, 1, 2, 2, 1, 2, 2, 2];
        println!("{}", Solution::can_jump(nums));

        let nums = vec![3, 0, 8, 2, 0, 0, 1];
        println!("{}", Solution::can_jump(nums));
    }
}
