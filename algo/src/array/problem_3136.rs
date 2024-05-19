#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(word: String) -> bool {
        let word = word.as_bytes();
        if word.len() < 3 {
            return false;
        }
        let mut a = 0;
        let mut v = 0;

        for i in word {
            match *i {
                b'u' | b'e' | b'o' | b'a' | b'i' | b'U' | b'E' | b'O' | b'A' | b'I' => {
                    a += 1;
                }
                b'a'..=b'z' | b'A'..=b'z' => {
                    v += 1;
                }
                b'0'..=b'9' => {}
                _ => return false,
            };
        }

        a >= 1 && v >= 1
    }
}
