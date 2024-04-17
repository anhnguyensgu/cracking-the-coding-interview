use std::usize;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        Self::recursive_with_memo(s)
    }

    fn recursive(s: String) -> bool {
        Self::do_recursive(s.as_bytes(), 0, 0)
    }

    fn do_recursive(s: &[u8], count: i32, i: usize) -> bool {
        if i == s.len() {
            count == 0
        } else {
            if count < 0 {
                return false;
            }
            println!("{i}:{count}");

            if s[i] == b'(' {
                Self::do_recursive(s, count + 1, i + 1)
            } else if s[i] == b')' {
                Self::do_recursive(s, count - 1, i + 1)
            } else {
                Self::do_recursive(s, count + 1, i + 1)
                    || Self::do_recursive(s, count - 1, i + 1)
                    || Self::do_recursive(s, count, i + 1)
            }
        }
    }

    fn recursive_with_memo(s: String) -> bool {
        let mut memo = vec![vec![-1; 100]; 100];
        Self::do_recursive_with_memo(s.as_bytes(), 0, 0, &mut memo)
    }

    fn do_recursive_with_memo(s: &[u8], count: i32, i: usize, memo: &mut Vec<Vec<i32>>) -> bool {
        if i == s.len() {
            count == 0
        } else {
            if count < 0 {
                return false;
            }

            if memo[i][count as usize] != -1 {
                return memo[i][count as usize] == 1;
            }
            let valid = match s[i] {
                b'(' => Self::do_recursive_with_memo(s, count + 1, i + 1, memo),
                b')' => Self::do_recursive_with_memo(s, count - 1, i + 1, memo),
                _ => {
                    Self::do_recursive_with_memo(s, count + 1, i + 1, memo)
                        || Self::do_recursive_with_memo(s, count - 1, i + 1, memo)
                        || Self::do_recursive_with_memo(s, count, i + 1, memo)
                }
            };

            memo[i][count as usize] = match valid {
                true => 1,
                false => 0,
            };
            valid
        }
    }
    fn dynamic_programming_bottom_up(s: String) -> bool {
        let chars = s.as_bytes();
        let mut dp = vec![vec![false; 200]; 200];
        dp[s.len()][0] = true;
        for i in (0..chars.len()).rev() {
            for j in 0..=chars.len() {
                dp[i][j] = match chars[i] {
                    b'(' => dp[i + 1][j + 1],
                    b')' if j > 0 => dp[i + 1][j - 1],
                    b'*' if j > 0 => dp[i + 1][j - 1] || dp[i + 1][j + 1] || dp[i + 1][j],
                    b'*' => dp[i + 1][j + 1] || dp[i + 1][j],
                    _ => false,
                };
            }
        }

        dp[0][0]
    }

    fn stack_base(s: String) -> bool {
        let mut open_stack = vec![];
        let mut star_stack = vec![];
        let chars = s.as_bytes();
        for i in 0..s.len() {
            match chars[i] {
                b'(' => {
                    open_stack.push(i);
                }
                b'*' => {
                    star_stack.push(i);
                }
                b')' if !open_stack.is_empty() => {
                    open_stack.pop();
                }

                b')' if !star_stack.is_empty() => {
                    star_stack.pop();
                }
                b')' if open_stack.is_empty() && star_stack.is_empty() => return false,
                _ => {}
            };
        }

        while !open_stack.is_empty() && !star_stack.is_empty() {
            if open_stack.pop() > star_stack.pop() {
                return false;
            }
        }

        open_stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_run() {
        let s = "()".into();
        let ans = Solution::stack_base(s);
        println!("{ans}");

        let s = "(*)".into();
        let ans = Solution::stack_base(s);
        println!("{ans}");

        let s = "(*))".into();
        let ans = Solution::stack_base(s);
        println!("{ans}");

        let s = "))((".into();
        let ans = Solution::stack_base(s);
        println!("{ans}");

        let s = "((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((".into();
        let ans = Solution::stack_base(s);
        println!("{ans}");
    }
}
