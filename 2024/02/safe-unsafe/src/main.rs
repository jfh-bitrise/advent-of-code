use std::fs::read_to_string;

fn main() {
    let lines = read_lines("./input.txt");

    let mut goodies = 0;
    let mut tolerated_goodies = 0;
    for line in lines {
        let parts: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();
        
        if analyze_vector(parts.clone()) {
            goodies += 1;
            tolerated_goodies += 1;
        } else {
            for i in 0..parts.len() {
                let mut tolerated_vec: Vec<i32> = Vec::with_capacity(parts.len()-1);
                for j in 0..parts.len() {
                    if i != j {
                        tolerated_vec.push(parts[j])
                    }
                }
                if analyze_vector(tolerated_vec.clone()) {
                    tolerated_goodies += 1;
                    break
                }
            }
        }
    }
    println!("The amount of good lines is: {}", goodies);
    println!("The amount of tolerated good lines is: {}", tolerated_goodies)
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}


fn analyze_vector(v: Vec<i32>) -> bool {
    for i in 1 .. v.len() {
        if !analyze(v[0], v[1], v[i-1], v[i]) {
            return false
        }
    }
    return true
}

fn analyze(f: i32, s: i32, a: i32, b: i32) -> bool {
    if f < s {
        if b - a <= 0 || b - a > 3 {
            return false
        }
    } else if f > s {
        if a - b <= 0 || a - b > 3 {
            return false
        }
    } else {
        return false
    }
    return true
} 