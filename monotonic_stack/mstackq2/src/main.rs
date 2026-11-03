fn main() {
    println!("Q2. Daily Temperatures");

    let tests = vec![
        TestCase{
            id: 1,
            temperatures: vec![73,74,75,71,69,72,76,73],
            expected: vec![1,1,4,2,1,1,0,0],
        },
        TestCase{
            id: 2,
            temperatures: vec![30,40,50,60],
            expected: vec![1,1,1,0],
        },
        TestCase{
            id: 3,
            temperatures: vec![30,60,90],
            expected: vec![1,1,0],
        },
    ];

    for test in tests {
        println!("\n TestCase {}: temperatures = {:?}", test.id, test.temperatures);
        let result = Solution::daily_temperatures(test.temperatures);
        println!("result: {:?}", result);
        assert_eq!(test.expected, result);
    }
}

struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut hotter: Vec<usize> = Vec::with_capacity(n);
        let mut answers: Vec<i32> = vec![0; n];
        for i in (0..n).rev() {
            // println!("start hotter loop");
            while let Some(&h) = hotter.last() {
                let ti = temperatures[i];
                let th = temperatures[h];
                // println!("temperatures[i={}]: {}, temperatures[h={}]: {}", i, ti, h, th);
                // println!("hotter: {:?}", hotter);
                // println!("answers: {:?}", answers);
                if ti < th {
                    answers[i] = (h - i) as i32;
                    hotter.push(i);
                    break;
                } else {
                    hotter.pop();
                }
            }
            // println!("end hotter loop");
            if hotter.len() == 0 {
                answers[i] = 0;
                hotter.push(i);
            }
        }
        answers
    }
}

struct TestCase {
    id: i32,
    temperatures: Vec<i32>,
    expected: Vec<i32>,
}
