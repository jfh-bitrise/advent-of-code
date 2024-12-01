use std::fs::read_to_string;

fn main() {
    let lines = read_lines("./input.txt");
    let mut first: Vec<i32> = vec![];
    let mut second: Vec<i32> = vec![];

    for line in lines {
        let parts = line.split("   ");
        let mut i = 0;
        for part in parts {
            if i == 0 {
                first.push(part.parse::<i32>().unwrap());
            } else {
                second.push(part.parse::<i32>().unwrap());
            }
            i += 1;
        }
    }

    first.sort();
    second.sort();

    let mut sum = 0;
    for i in 0..first.len() {
        sum += (first[i]-second[i]).abs();
        // println!("{} {} {}", first[i], second[i], (first[i]-second[i]).abs())
    }
    println!("{}", sum)

}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
