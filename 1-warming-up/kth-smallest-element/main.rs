///
/// Kth smallest element
/// 
/// See: https://www.hackerrank.com/challenges/fizzbuzz/problem
use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut arr: [i32; 6] = [7, 10, 4, 3, 20, 15];
    let position: usize = args[1].to_string().parse::<usize>().unwrap();

    arr.sort();


    println!("{:?}", arr);
    println!("{} smallest number: {:?}", position, arr[position - 1]);
}