struct Solution {}

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        // use std::collections::HashMap;
        // // let a = HashMap::new();
        // for (i, a) in arr1.windows(2).enumerate() {
        //
        // }
        vec![]
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_run() {
        let arr = [10, 20, 30, 40, 50];

        // Iterate over the array with index
        for (i, window) in arr.windows(2).enumerate() {
            println!(
                "arr[{}] = {}, arr[{} - 1] = {}",
                i + 1,
                window[1],
                i + 1,
                window[0]
            );
        }
    }
}
