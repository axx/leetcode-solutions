fn main() {
    println!("Q3. Largest Rectangle in Histogram");

    let tests = vec![
        TestCase {
            id: 1,
            heights: vec![2, 1, 5, 6, 2, 3],
            expected: 10,
        },
        TestCase {
            id: 2,
            heights: vec![2, 4],
            expected: 4,
        },
        TestCase {
            id: 3,
            heights: vec![2, 1, 0, 2],
            expected: 2,
        },
        TestCase {
            id: 4,
            heights: vec![1, 1],
            expected: 2,
        },
        TestCase {
            id: 5,
            heights: vec![5, 4, 1, 2],
            expected: 8,
        },
    ];

    for test in tests {
        println!("\nTestCase {}: heights={:?}", test.id, test.heights);
        let output = Solution::largest_rectangle_area(test.heights);
        println!("output: {}", output);
        assert_eq!(output, test.expected);
    }
}

struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut stack = Vec::with_capacity(n);
        let mut max_width = n;
        let mut answer = 0;

        for i in (0..n).rev() {
            while let Some(&j) = stack.last() {
                if heights[j] >= heights[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if heights[i] == 0 {
                max_width = i;
                continue;
            }
            stack.push(i); // The current height is the highest in the stack

            println!("i = {}, stack = {:?}", i, stack);

            // loop through stack to calculate areas from the current height i
            for j in 0..stack.len() {
                let area;
                if j == 0 {
                    // smallest height in stack
                    area = heights[stack[j]] * (max_width - i) as i32;
                } else {
                    area = heights[stack[j]] * (stack[j-1] - i) as i32;
                }
                println!("j = {}, area = {}", j, area);
                if area > answer {
                    answer = area;
                    println!("updated answer = {}", answer);
                }
            }
        }

        answer
    }
}

struct TestCase {
    id: usize,
    heights: Vec<i32>,
    expected: i32,
}
