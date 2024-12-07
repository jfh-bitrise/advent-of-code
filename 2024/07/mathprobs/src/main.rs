use std::fs::read_to_string;

fn main() {
    let lines = read_lines("./input.txt");
    
    let mut sum: i128 = 0;

    let mut results: Vec<i64> = Vec::new();
    let mut nums: Vec<Vec<i64>> = Vec::new();
    for line in lines {
        let l: Vec<&str> = line.split(':').collect();
        results.push(l[0].parse::<i64>().unwrap());
        let n: Vec<&str> = l[1].split_whitespace().collect();
        let mut nn: Vec<i64> = Vec::new();
        for i in 0..n.len() {
            nn.push(n[i].parse::<i64>().unwrap())
        }
        nums.push(nn);
    }

    for i in 0..results.len() {
        if check(results[i], nums[i].clone()) {
            sum += results[i] as i128;
        }
    }
    println!("Good results: {}", sum)
}

fn check(r: i64, n: Vec<i64>) -> bool {
    let methods = vec!["add", "mul", "com"];

    let mut combs: Vec<Vec<&str>> = Vec::new();

    for i in 0..methods.len().pow((n.len()-1) as u32) {
        let mut c : Vec<&str> = Vec::new();
        let mut temp = i;

        for _ in 0..n.len()-1 {
            c.push(&methods[temp % methods.len()]);
            temp /= methods.len();
        }

        combs.push(c);
    }


    for c in combs {
        let mut res = n[0];
        for i in 0..n.len()-1 {
            if c[i] == "add" {
                res += n[i+1]
            } 
            if c[i] == "mul" {
                res *= n[i+1];
            }
            if c[i] == "com" {
                let pow_val = ((n[i+1] + 1) as f64).log10().ceil() as u32;
                res = res * (10 as i64).pow(pow_val) + n[i+1]
            }
            if res > r {
                break
            }
        }
        if res == r {
            return true
        }
    }

    // println!("Miss: {}: {:?}", r, n);

    return false
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
