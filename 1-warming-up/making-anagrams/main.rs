 //
 // Strings: Making Anagrams
 //
 // See: https://www.hackerrank.com/challenges/ctci-making-anagrams


 use std::vec::Vec;

 fn make_anagram(a: &str, b: &str) -> i32 {
     let mut count: i32 = 0;
     let mut frequency: Vec<i32> = vec![0; 26];

     for c in a.chars() {
         frequency[(c as u32 - 'a' as u32) as usize] += 1;
     }

     for c in b.chars() {
         frequency[(c as u32 - 'a' as u32) as usize] -= 1;
     }

     for x in frequency {
         count += x.abs()
     }

     return count;
 }

 fn main(){
     let a = String::from("cde");
     let b = String::from("abc");
     println!("Changes: {}", make_anagram(&a, &b));
 }