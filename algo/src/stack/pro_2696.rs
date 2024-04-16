struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_length_replace(s: String) -> i32 {
        let mut a = s.clone();
        let mut cur = a.len();
        let mut last = 0;
        while cur != last {
            a = a.replace("AB", "").replace("CD", "");
            last = cur;
            cur = a.len();
        }
        a.len() as i32
    }

    #[allow(dead_code)]
    pub fn min_length(s: String) -> i32 {
        let b = s.as_bytes();
        let mut stack: Vec<&u8> = vec![];
        for i in b {
            match stack.last() {
                Some(last) if **last == b'A' && *i == b'B' => {
                    stack.pop();
                }
                Some(last) if **last == b'C' && *i == b'D' => {
                    stack.pop();
                }
                _ => stack.push(i),
            };
        }

        stack.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_min_length() {
        let ans = Solution::min_length("ABFCACDB".into());
        println!("{ans}");
        let ans = Solution::min_length("ACBBD".into());
        println!("{ans}");
    }
}
