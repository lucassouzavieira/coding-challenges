//
// 657. Robot Return to Origin
// 
// See: https://leetcode.com/problems/robot-return-to-origin/

use std::string::String;

#[derive(Debug)]
pub struct Tracker {
    vertical: i32,
    horizontal: i32,
}

impl Tracker {
    pub fn judge_moviment(&mut self, moves: &String) -> bool {
        for movement in moves.chars() {
            match movement {
                'R' => self.horizontal += 1,
                'L' => self.horizontal -= 1,
                'U' => self.vertical += 1,
                'D' => self.vertical -= 1,
                _ => println!("Unknow moviment")
            }    
        }

        return self.horizontal == 0 && self.vertical == 0
    }
}


fn main(){
    let mut tracker = Tracker{vertical: 0, horizontal: 0};
    let moves: String = String::from("RLL");
    let result = tracker.judge_moviment(&moves);
    println!("{}", result);
}