#[cfg(test)]
mod test {
    #[test]
    fn run() {
        let chalk = vec![5, 1, 5];
        let k = 27;
        let ans = chalk_replacer(chalk, k);
        println!("{ans}");
    }
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let sum: i64 = chalk.iter().map(|&a| a as i64).sum();
        let mut remaining = k as i64 % sum;
        let Some((ans, _)) = chalk.iter().enumerate().find(|(_, &c)| {
            let c = c as i64;
            if c > remaining {
                return true;
            }
            remaining -= c;
            false
        }) else {
            return -1;
        };

        ans as i32
    }
}
