fn main() {
    println!("Q2. Valid Mountain Array");
    let test_cases = vec![
        TestCase { arr: vec![2,1], expected: false },
        TestCase { arr: vec![3,5,5], expected: false },
        TestCase { arr: vec![0,3,2,1], expected: true },
    ];
    for (i, tc) in test_cases.iter().enumerate() {
        let result = Solution::valid_mountain_array(tc.arr.clone());
        println!("Test case {}: {:?}, {}", i+1, tc.arr, result);
        assert_eq!(result, tc.expected);
    }
}

struct TestCase {
    arr: Vec<i32>,
    expected: bool,
}

struct Solution;

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let n = arr.len();
        if n < 3 {
            return false;
        }
        let mut pr = n-1;
        let mut pl = 0;
        let mut rmove = true;
        let mut lmove = true;
        for i in 1..n-1 {
            if lmove && arr[pl] < arr[i] {
                pl = i;
            } else {
                lmove = false;
            }
            if rmove && arr[n-1-i] > arr[pr] {
                pr = n-1-i;
            } else {
                rmove = false;
            }
            if !lmove && !rmove {
                break;
            }
        }
        pr == pl
    }
}
