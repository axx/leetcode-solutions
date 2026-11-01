fn main() {
    println!("Q1. Build an Array With Stack Operations");

    let target = vec![1, 3];
    let n = 3;
    println!("{:?}", Solution::build_array(target, n));

    let target = vec![1, 2, 3];
    let n = 3;
    println!("{:?}", Solution::build_array(target, n));

    let target = vec![1, 2];
    let n = 4;
    println!("{:?}", Solution::build_array(target, n));
}

struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut ops = Vec::with_capacity(n as usize * 2);
        let mut stack = Vec::with_capacity(target.len());
        let mut counter = 0;

        // Loop through the stream of n integers
        for i in 1..=n {
            ops.push("Push".to_string());
            if i == target[counter] {
                stack.push(i);
                counter += 1;
            } else {
                ops.push("Pop".to_string());
            }
            if counter == target.len() {
                break;
            }
        }

        ops
    }
}