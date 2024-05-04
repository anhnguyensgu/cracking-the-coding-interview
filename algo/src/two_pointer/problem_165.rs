#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let version1: Vec<_> = version1.split(".").collect();
        let version2: Vec<_> = version2.split(".").collect();
        let mut i = 0usize;
        let mut j = 0usize;
        while i < version1.len() || j < version2.len() {
            let v1 = if i >= version1.len() {
                0usize
            } else {
                version1[i].parse().unwrap()
            };

            let v2 = if j >= version2.len() {
                0usize
            } else {
                version2[j].parse().unwrap()
            };

            if v1 > v2 {
                return 1;
            }

            if v1 < v2 {
                return -1;
            }
            i += 1;
            j += 1;
        }

        0
    }
}
