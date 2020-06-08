//
// Sum of Even Numbers After Queries
// 
// See: https://leetcode.com/problems/sum-of-even-numbers-after-queries/

fn sum_even_numbers(arr: &Vec<i32>) -> i32 {
    let mut value: i32 = 0;
    
    for x in arr {
        if x % 2 == 0 {
            value += x;
        }
    }

    return value;
}

fn sum_even_array(arr: &Vec<i32>, queries: &Vec<Vec<i32>>) -> Vec<i32> {    
    let mut result: Vec<i32> = vec![];
    let mut modified_array: Vec<i32> = arr.clone();
    let mut cumulative_even: i32 = 0;

    cumulative_even = sum_even_numbers(&modified_array);

    for query in queries {
        let value: i32 = query[0];
        let index: usize = query[1] as usize;

        if modified_array[index] % 2 == 0 {
            cumulative_even -= modified_array[index]
        }

        modified_array[index] += value;

        if modified_array[index] % 2 == 0 {
            cumulative_even += modified_array[index]
        }

        result.push(cumulative_even);
    }

    result
}

fn main(){
    let arr: Vec<i32> = vec![1, 2, 3, 4];
    let queries: Vec<Vec<i32>> = vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]];

    println!("Result {:?}", sum_even_array(&arr, &queries));
}