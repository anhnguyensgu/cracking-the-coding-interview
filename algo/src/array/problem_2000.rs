#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let chars = word.as_bytes();
        let i = 0;
        let mut j = -1;
        for (idx, c) in chars.iter().enumerate() {
            if ch as u8 == *c {
                j = idx as i32;
                break;
            }
        }

        if j == -1 {
            return word;
        }

        let mut reverse = vec![];
        for idx in (i..=j).rev() {
            reverse.push(chars[idx as usize]);
        }

        for idx in (j + 1) as usize..word.len() {
            reverse.push(chars[idx]);
        }

        String::from_utf8_lossy(reverse.as_slice()).into()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn run() {
        let ans = Solution::reverse_prefix("abcdefd".into(), 'd');
        println!(" {ans} ");
    }
}
