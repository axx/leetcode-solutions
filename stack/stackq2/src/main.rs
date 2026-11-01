fn main() {
    println!("Q2. Evaluate Reverse Polish Notation");

    let tokens: Vec<String> = to_string_vec(&["2","1","+","3","*"]);
    assert_eq!(Solution::eval_rpn(tokens), 9);

    let tokens: Vec<String> = to_string_vec(&["4","13","5","/","+"]);
    assert_eq!(Solution::eval_rpn(tokens), 6);

    let tokens: Vec<String> = to_string_vec(&["10","6","9","3","+","-11","*","/","*","17","+","5","+"]);
    assert_eq!(Solution::eval_rpn(tokens), 22);
}

fn to_string_vec(arr: &[&str]) -> Vec<String> {
    arr.iter().map(|s| s.to_string()).collect()
}

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut values = Vec::with_capacity(tokens.len());
        for token in tokens {
            match token.as_str() {
                "+" => {
                    let v2 = values.pop().unwrap();
                    let v1 = values.pop().unwrap();
                    values.push(v1 + v2);
                },
                "-" => {
                    let v2 = values.pop().unwrap();
                    let v1 = values.pop().unwrap();
                    values.push(v1 - v2);
                },
                "*" => {
                    let v2 = values.pop().unwrap();
                    let v1 = values.pop().unwrap();
                    values.push(v1 * v2);
                },
                "/" => {
                    let v2 = values.pop().unwrap();
                    let v1 = values.pop().unwrap();
                    values.push(v1 / v2);
                },
                _ => {
                    let v: i32 = token.parse().unwrap();
                    values.push(v);
                },
            }
        }
        values.pop().unwrap()
    }
}
