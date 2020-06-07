 //
 // Nth Fibonacci
 //
 // See: https://www.algoexpert.io/questions/Nth%20Fibonacci

 use std::vec::Vec;

 fn fibonacci(n: u32) -> u32 {
    let mut sequence: Vec<u32> = vec![0; (n + 2) as usize];
    sequence[0] = 0;
    sequence[1] = 1;

    let mut i: u32 = 2;

    while i <= n {
        sequence[i as usize] = sequence[(i - 1) as usize] + sequence[(i - 2) as usize];
        i += 1;
    }

    return sequence[(n - 1) as usize];
 }

 fn main(){
     println!("Fibonacci {}th: {}", 6, fibonacci(6));
 }