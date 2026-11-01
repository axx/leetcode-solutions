fn main() {
    println!("Q3. Find All Numbers Disappeared in an Array");

    let v = vec![4, 3, 2, 7, 8, 2, 3, 1];
    println!("{:?}", Solution::find_disappeared_numbers(v));

    let v = vec![1, 1];
    println!("{:?}", Solution::find_disappeared_numbers(v));

    let v = vec![2, 2];
    println!("{:?}", Solution::find_disappeared_numbers(v));
}

struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut found = vec![0; n];
        let mut result = Vec::with_capacity(n);
        for &num in nums.iter() {
            found[(num - 1) as usize] = 1;
        }
        for i in 1..=n {
            if found[i-1] == 0 {
                result.push(i as i32);
            }
        }
        result
    }
}
