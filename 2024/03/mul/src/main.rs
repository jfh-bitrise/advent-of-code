use std::fs::read_to_string;
use regex::Regex;


fn main() {
    let lines = read_lines("./input.txt");
    let re = Regex::new(r"mul\((\d*),(\d*)\)|do\(\)|don\'t\(\)").unwrap();


    let mut sum = 0;
    let mut advanced_sum = 0;
    let mut do_cal = true;
    for line in lines {
        for cap in re.captures_iter(&line) {
            let m = cap.get(0).map_or("", |m| m.as_str());

            match m {
                "do()" => do_cal = true,
                "don't()" => do_cal = false,
                _ => {
                    let d1 = cap.get(1).map_or("", |m| m.as_str());
                    let d2 = cap.get(2).map_or("", |m| m.as_str());
                    println!("{} {}", d1, d2);
                    if let (Ok(num1), Ok(num2)) = (d1.parse::<i32>(), d2.parse::<i32>()) {
                        sum += num1 * num2;
                        if do_cal {
                            advanced_sum += num1 * num2;
                        }
                    }
                }
            }
            
        }
    }
    println!("The sum: {}", sum);
    println!("The advanced sum: {}", advanced_sum);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
