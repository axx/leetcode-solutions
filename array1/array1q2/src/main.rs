fn main() {
    println!("Array 1 Q2: Shuffle the Array");
    let result = Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3);
    println!("{:?}", result);
}

struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = nums.len()/2;
        let x = &nums[0..n];
        let y = &nums[n..2*n];
        let mut ans = Vec::with_capacity(2*n);
        for i in 0..n {
            ans.push(x[i]);
            ans.push(y[i]);
        }
        ans
    }
}