fn main() {
    println!("Q3. Max Consecutive Ones");
    let result = Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]);
    println!("{:?}", result);
}

struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_count = 0;
        let mut count = 0;
        for v in nums {
            if v == 1 {
                count += 1;
                if count > max_count {
                    max_count = count;
                }
            } else {
                count = 0;
            }
        }
        max_count
    }
}
