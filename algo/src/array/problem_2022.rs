#[cfg(test)]
mod test {

    #[test]
    fn run() {
        let original = vec![1, 2, 3, 4, 5];
        let m = 3;
        let n = 2;
        let ans = construct2_d_array(original, m, n);
        println!("{ans:?}");
    }
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        use std::*;
        let mut a = true;
        let ans = original
            .chunks(n as usize)
            .fold((vec![], true), |mut acc, i| {
                acc.0.push(i.to_vec());
                if i.len() != n as usize {
                    acc.1 = false;
                    a = false;
                }
                acc
            });
        if !a || ans.0.len() != m as usize {
            return vec![];
        }
        ans.0
    }
    #[derive(Debug)]
    struct Abc {
        val: i32,
    }
    #[test]
    fn test_ref() {
        let abc = Abc { val: 1 };
        let sum = |a: Abc| -> Abc {
            let mut a = a;
            a.val += 1;
            a
        };
        let abc = run_twice(sum, abc);

        println!("{abc:?}");
    }
    fn run_twice<F>(func: F, abc: Abc) -> Abc
    where
        F: Fn(Abc) -> Abc,
    {
        func(func(abc))
    }
}
