 //
 // Array Left Rotation
 //
 // See: https://www.hackerrank.com/challenges/ctci-array-left-rotation/problem
 //

 use std::vec::Vec;

 fn left_rotation(arr: &Vec<i32>, rotations: i32) {
    let mut rotated_vector = arr.clone();

    let position = |index: usize, rotations: i32| -> usize {
        let rot: usize = rotations.to_string().parse::<usize>().unwrap();
        return (index + (arr.len() - rot)) % arr.len();
    };

    for (i, x) in arr.iter().enumerate() {
        rotated_vector[position(i, rotations)] = *x;
    }

    println!("Rotated array: {:?}", rotated_vector);
 }

 fn main(){
    let arr: Vec<i32> = vec![1, 2, 3, 4, 5];
    let rotations: i32 = 4;
    left_rotation(&arr, rotations);
}