//
// Subarray sum equals K
// 
// See: https://leetcode.com/problems/subarray-sum-equals-k/

use std::collections::HashMap;

fn subarray_sum(nums: &Vec<i32>, k: i32) -> i32 {
    let mut sums = HashMap::new();
    let mut combinations: i32 = 0;
    let mut current_sum: i32 = 0;

    sums.insert(0, 1);

    for num in nums {
        current_sum = current_sum + num;
        
        if let Some(now) = sums.get(&(current_sum - k)) {
            combinations += now.clone();
        }

        sums.insert(current_sum, sums.get(&current_sum).unwrap_or(&0) + 1);
    }

    return combinations;
}

fn main(){
    let arr: Vec<i32> = vec![3, 4, 7, 2, -3, 1, 4, 2, 1];
    let k: i32 = 7;

    let combinations = subarray_sum(&arr, k);
    println!("You can sum the value {} with {} combinations using array {:?}", k, combinations, arr);
}