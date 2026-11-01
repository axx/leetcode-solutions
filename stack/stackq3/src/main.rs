fn main() {
    println!("Q3. Exclusive Time of Functions");

    let tests = vec![
        TestCase {
            id: 1,
            n: 1,
            logs: to_string_vec(&["0:start:0", "0:end:0"]),
            expected: vec![1],
            debug: false,
        },
        TestCase {
            id: 2,
            n: 2,
            logs: to_string_vec(&["0:start:0", "1:start:2", "1:end:5", "0:end:6"]),
            expected: vec![3, 4],
            debug: false,
        },
        TestCase {
            id: 3,
            n: 1,
            logs: to_string_vec(&["0:start:0","0:start:2","0:end:5","0:start:6","0:end:6","0:end:7"]),
            expected: vec![8],
            debug: false,
        },
        TestCase {
            id: 4,
            n: 2,
            logs: to_string_vec(&["0:start:0","0:start:2","0:end:5","1:start:6","1:end:6","0:end:7"]),
            expected: vec![7,1],
            debug: false,
        },
        TestCase {
            id: 5,
            n: 2,
            logs: to_string_vec(&["0:start:0","0:start:2","0:end:5","1:start:7","1:end:7","0:end:8"]),
            expected: vec![8,1],
            debug: false,
        },
        TestCase {
            id: 6,
            n: 1,
            logs: to_string_vec(&["0:start:0","0:start:1","0:start:2","0:end:3","0:end:4","0:end:5"]),
            expected: vec![6],
            debug: false,
        },
        TestCase {
            id: 7,
            n: 2,
            logs: to_string_vec(&["0:start:0","0:end:1","1:start:5","1:end:6"]),
            expected: vec![2,2],
            debug: true,
        },
    ];

    for test in tests {
        println!("\n\n####### Test Case {} #########################################", test.id);
        println!("Running test with n = {}, logs = {:?}", test.n, test.logs);
        let result = Solution::exclusive_time(test.n, test.logs, test.debug);
        println!("Result: {:?}", result);
        assert_eq!(result, test.expected);
    }

}

struct TestCase {
    id: usize,
    n: i32,
    logs: Vec<String>,
    expected: Vec<i32>,
    debug: bool,
}

fn to_string_vec(v: &[&str]) -> Vec<String> {
    v.into_iter().map(|s| s.to_string()).collect()
}

struct Solution;

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>, debug: bool) -> Vec<i32> {
        let mut prev_entry: Option<LogEntry> = None;
        let mut exclusive_times = vec![0; n as usize];
        let mut prev_end = 0;
        let mut fid_stack: Vec<i32> = Vec::with_capacity(logs.len());

        for log in logs {
            let entry = LogEntry::parse(log.as_str());
            if debug {
                println!("\nProcessing log: {}", log);
            }

            if let Some(&prev) = prev_entry.as_ref() {
                if prev.is_start && entry.is_start {
                    prev_end = entry.timestamp - 1;
                    exclusive_times[prev.fid as usize] += prev_end - prev.timestamp + 1;
                    fid_stack.push(entry.fid);
                } else if prev.is_start && (!entry.is_start) {
                    exclusive_times[entry.fid as usize] += entry.timestamp - prev.timestamp + 1;
                    prev_end = entry.timestamp;
                    fid_stack.pop();
                } else if (!prev.is_start) && (!entry.is_start) {
                    exclusive_times[entry.fid as usize] += entry.timestamp - prev_end;
                    prev_end = entry.timestamp;
                    fid_stack.pop();
                } else if (!prev.is_start) && entry.is_start {
                    if (entry.timestamp > prev_end + 1) && (fid_stack.len()>0) {
                        let prev_fid = fid_stack.last().unwrap();
                        exclusive_times[*prev_fid as usize] += entry.timestamp - prev_end - 1;
                    }
                    fid_stack.push(entry.fid);
                }
            } else {
                fid_stack.push(entry.fid);
            }
            prev_entry = Some(entry);

            if debug {
                println!("Current exclusive times: {:?}", exclusive_times);
                println!("Current fid stack: {:?}", fid_stack);
                println!("Previous entry: {:?}", prev_entry);
                println!("Current entry: {:?}", entry);
                println!("Current prev_end: {:?}", prev_end);
            }
        }

        exclusive_times
    }
}

#[derive(Clone, Copy, Debug)]
struct LogEntry {
    fid: i32,
    timestamp: i32,
    is_start: bool,
}

impl LogEntry {
    pub fn parse(log: &str) -> Self {
        let parts: Vec<&str> = log.split(':').collect();
        let fid = parts[0].parse::<i32>().unwrap();
        let is_start = parts[1] == "start";
        let timestamp = parts[2].parse::<i32>().unwrap();

        LogEntry {
            fid,
            timestamp,
            is_start,
        }
    }
}
