use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i64> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let r1 = nums[0];
        let s = nums[1];

        let r2 = 2 * s - r1;

        println!("{}", r2);
    }
}