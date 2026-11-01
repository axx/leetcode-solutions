fn main() {
    println!("Q1. Final Prices With a Special Discount in a Shop");

    let tests = vec![
        TestCase {
            id: 1,
            prices: vec![8,4,6,2,3],
            expected: vec![4,2,4,2,3],
        },
        TestCase {
            id: 2,
            prices: vec![1,2,3,4,5],
            expected: vec![1,2,3,4,5],
        },
        TestCase {
            id: 3,
            prices: vec![10,1,1,6],
            expected: vec![9,0,1,6],
        },
    ];

    for test in tests {
        println!("\n\n####### Test Case {} #########################################", test.id);
        println!("Running test with prices = {:?}", test.prices);
        let result = Solution::final_prices(test.prices.clone());
        println!("Result: {:?}", result);
        assert_eq!(result, test.expected);
    }

}

struct Solution;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let n = prices.len();
        let mut result = prices.clone();
        for i in 0..n {
            for j in (i+1)..n {
                if prices[j] <= prices[i] {
                    result[i] -= prices[j];
                    break;
                }
            }
        }
        result
    }
}

struct TestCase {
    id: usize,
    prices: Vec<i32>,
    expected: Vec<i32>,
}
