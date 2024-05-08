mod problem_122;
mod problem_55;
mod problem_853;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        use std::collections::*;
        let mut people_map: BTreeMap<i32, i32> = BTreeMap::new();
        for &e in &people {
            *people_map.entry(e).or_insert(0) += 1;
        }

        let mut ans = 0;
        while !people_map.is_empty() {
            let mut cur = 0;
            let mut c = 0;

            while c < 2 && !people_map.is_empty() && cur < limit {
                c += 1;
                let (w, count) = match people_map.range(..=(limit - cur)).next_back() {
                    Some((count, value)) => (count.clone(), value.clone()),
                    None => break,
                };

                match count {
                    1 => {
                        people_map.remove(&w);
                    }
                    _ => {
                        let e = people_map.get_mut(&w).unwrap();
                        *e -= 1;
                    }
                }
                cur += w;
            }
            ans += 1;
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn run() {
        let boats = vec![1, 2];
        let limit = 3;
        let ans = Solution::num_rescue_boats(boats, limit);
        let expected = 1;
        assert_eq!(ans, expected);

        let boats = vec![3, 2, 2, 1];
        let limit = 3;
        let ans = Solution::num_rescue_boats(boats, limit);
        let expected = 3;
        assert_eq!(ans, expected);

        let boats = vec![3, 5, 3, 4];
        let limit = 5;
        let ans = Solution::num_rescue_boats(boats, limit);
        let expected = 4;
        assert_eq!(ans, expected);

        let boats = vec![3, 2, 3, 2, 2];
        let limit = 6;
        let ans = Solution::num_rescue_boats(boats, limit);
        let expected = 3;
        assert_eq!(ans, expected);
    }
}
