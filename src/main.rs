use std::fs::read_to_string;
use regex::Regex;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    assert!(args.len() > 2);
    let filename = &args[1];
    let user_expr = &args[2];

    let expr = Regex::new(user_expr).unwrap();
    let contents = read_lines(filename);

    let mut target_level: i32 = -1;
    let mut flag = false;

    for s in contents {

        let current_level: i32 = level(&s);

        // check whether it's time to stop
        if flag && current_level == target_level {
            break;
        }

        // find starting place
        if expr.is_match(&s) {
            target_level = level(&s);
            flag = true;
            if target_level == 0 {
                println!("no title matching '{}' found", expr);
                break;
            }
        }

        // decide whether to print
        if target_level != -1 {
            if current_level == 0 || current_level >= target_level {
                println!("{}", s);
            } else {
                break;
            }
        }
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn level(s: &str) -> i32 {
    let mut count = 0;
    for c in s.chars() {
        if c == '#' {
            count += 1;
        } else {
            break;
        }
    }
    count
}

