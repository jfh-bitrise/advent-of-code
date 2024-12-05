use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let lines = read_lines("./input.txt");

    let mut s1 = true;

    let mut rules: Vec<Vec<i32>> = Vec::new();
    let mut sum = 0;
    let mut bad_sum = 0;

    for line in lines {
        if line == "" {
            s1 = false
        } else if s1 {
            let rule: Vec<i32> = line.split("|").filter_map(|s| s.parse::<i32>().ok()).collect();
            rules.push(rule)
        } else {
            let mut ch: Vec<i32> = line.split(",").filter_map(|s| s.parse::<i32>().ok()).collect();
            let (goodie, s1, s2) = check_line(&ch, &rules);
            if goodie {
                sum += ch[ch.len()/2];
            } else {
                recursive_swap_and_check(&mut ch, &rules, &mut bad_sum, s1, s2);
            }
        }
    }

    println!("Sum sum: {}", sum);
    println!("Bad sum sum: {}", bad_sum)
}

fn recursive_swap_and_check(
    ch: &mut Vec<i32>,
    rules: &Vec<Vec<i32>>,
    bad_sum: &mut i32,
    s1: usize,
    s2: usize,
) {
    ch.swap(s1, s2);

    let (goodie, new_s1, new_s2) = check_line(ch, rules);
    if goodie {
        *bad_sum += ch[ch.len() / 2];
        return
    } else {
        recursive_swap_and_check(ch, rules, bad_sum, new_s1, new_s2);
    }
}

fn check_line(ch: &Vec<i32>, rules: &Vec<Vec<i32>>) -> (bool, usize, usize) {
    let mut change: HashMap<i32, usize> = HashMap::new();

    for i in 0..ch.len() {
        change.insert(ch[i], i);
    }

    for r in rules.clone() {
        if let Some(&v1) = change.get(&r[0]) {
            if let Some(&v2) = change.get(&r[1]) {
                if v2 < v1 {
                    return (false, v1, v2)
                }
            }
        }
    }
    return (true, 0, 0)
}


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

