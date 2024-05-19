use std::usize;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
        use std::collections::*;
        let mut ans = 0;
        let k = k as usize;
        let word = word.as_bytes();
        let mut i = 0usize;
        let mut frequency = HashMap::new();
        while i <= word.len() - k {
            *frequency.entry(&word[i..i + k]).or_insert(0) += 1;
            i += k;
        }

        let mut max_frequen = (&word[0..1], 0);

        for (s, c) in frequency {
            if c > max_frequen.1 {
                max_frequen = (s, c)
            }
        }

        i = 0;
        while i <= word.len() - k {
            if &word[i..i + k] != max_frequen.0 {
                ans += 1;
            }
            i += k;
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn run() {
        let word = "leetcodeleet".to_string();
        let k = 4;
        let ans = Solution::minimum_operations_to_make_k_periodic(word, k);
        println!("{ans}");

        let word = "leetcoleet".to_string();
        let k = 2;
        let ans = Solution::minimum_operations_to_make_k_periodic(word, k);
        println!("{ans}");
    }
}
