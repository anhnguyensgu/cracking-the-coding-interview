use std::usize;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut memo = vec![vec![-1; 100]; 100];
        Self::recursive_with_memo(s.as_bytes(), 0, 0, &mut memo)
    }

    fn recursive(s: &[u8], count: i32, i: usize) -> bool {
        if i == s.len() {
            return count == 0;
        } else {
            if count < 0 {
                return false;
            }
            println!("{i}:{count}");

            if s[i] == b'(' {
                return Self::recursive(s, count + 1, i + 1);
            } else if s[i] == b')' {
                return Self::recursive(s, count - 1, i + 1);
            } else {
                return Self::recursive(s, count + 1, i + 1)
                    || Self::recursive(s, count - 1, i + 1)
                    || Self::recursive(s, count, i + 1);
            }
        }
    }

    fn recursive_with_memo(s: &[u8], count: i32, i: usize, memo: &mut Vec<Vec<i32>>) -> bool {
        if i == s.len() {
            return count == 0;
        } else {
            if count < 0 {
                return false;
            }

            if memo[i][count as usize] != -1 {
                return memo[i][count as usize] == 1;
            }
            let mut valid = false;

            if s[i] == b'(' {
                valid = Self::recursive_with_memo(s, count + 1, i + 1, memo);
            } else if s[i] == b')' {
                valid = Self::recursive_with_memo(s, count - 1, i + 1, memo);
            } else {
                valid = Self::recursive_with_memo(s, count + 1, i + 1, memo)
                    || Self::recursive_with_memo(s, count - 1, i + 1, memo)
                    || Self::recursive_with_memo(s, count, i + 1, memo);
            }
            memo[i][count as usize] = match valid {
                true => 1,
                false => 0,
            };
            return valid;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_run() {
        let s = "()".into();
        let ans = Solution::check_valid_string(s);
        println!("{ans}");

        let s = "(*)".into();
        let ans = Solution::check_valid_string(s);
        println!("{ans}");

        let s = "(*))".into();
        let ans = Solution::check_valid_string(s);
        println!("{ans}");

        let s = "))((".into();
        let ans = Solution::check_valid_string(s);
        println!("{ans}");

        let s = "****".into();
        let ans = Solution::check_valid_string(s);
        println!("{ans}");
    }
}
