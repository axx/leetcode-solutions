use std::{assert_eq, println};

fn main() {
    println!("Q1. Plus One");

    let tests = vec![
        TestCase {
            id: 1,
            digits: vec![1,2,3],
            expected: vec![1,2,4],
        },
    ];

    for t in tests {
        let output = Solution::plus_one(t.digits);
        println!("Test {}: output: {:?}", t.id, output);
        assert_eq!(output, t.expected);
    }
}

struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut d = digits.clone();
        let mut carry = 0;
        for i in (0..d.len()).rev() {
            if d[i] == 9 {
                d[i] = 0;
                carry = 1;
            } else {
                d[i] += 1;
                carry = 0;
                break;
            }
        }
        if carry == 1 {
            d.insert(0, 1);
        }
        d
    }
}

struct TestCase {
    id: usize,
    digits: Vec<i32>,
    expected: Vec<i32>,
}