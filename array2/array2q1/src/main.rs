fn main() {
    println!("Array 2 Q1. Set Mismatch");
    let result = Solution::find_error_nums(vec![1, 2, 2, 4]);
    println!("{:?}", result);
}

struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut tracker = vec![0; n];
        let mut ans = vec![0, 0];
        for num in nums {
            tracker[(num - 1) as usize] += 1;
        }

        for i in 1..=n {
            let count = tracker[(i - 1) as usize];
            if count == 2 {
                ans[0] = i as i32;
            } else if count == 0 {
                ans[1] = i as i32;
            }
        }
        ans
    }
}
