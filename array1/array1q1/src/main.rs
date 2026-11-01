fn main() {
    println!("Array 1 Q1: Concatenation of Array");
    let result = Solution::get_concatenation(vec![1, 2, 3]);
    println!("{:?}", result);
}

struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans: Vec<i32> = vec![0; n * 2];
        for i in 0..nums.len() {
            ans[i] = nums[i];
            ans[i + n] = nums[i];
        }
        ans
    }
}
