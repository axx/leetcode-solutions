fn main() {
    println!("Q2. How Many Numbers Are Smaller Than the Current Number?");
    println!("{:?}", Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]));
}

struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();
        let mut result = vec![0; nums.len()];
        for (i, num) in nums.iter().enumerate() {
            let count = sorted_nums.iter().position(|x| x == num).unwrap();
            result[i] = count as i32;
        }
        result
    }
}