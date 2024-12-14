
use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, Write};



fn main() -> io::Result<()> {
    let lines = read_lines("./input.txt");
    let width = 101;
    let height = 103;
    let iterations = 5562;  

    // let mut q1c = 0;
    // let mut q2c = 0;
    // let mut q3c = 0;
    // let mut q4c = 0;

    // (103) 836 | 939 | 1042 | 1145 | 1248 | 1351
    // (101) 843 - 944 - 1045 - 1146 - 1247 - 1348

    // 843 + x*101 = 836 + y*103
    // 7 + x*101 = y*103
    // 7/103 + (x*101/103) = y

    let mut x = 0;
    loop {
        x += 1;
        if ((x * 101) + 7) % 103 == 0 {
            // println!("X: {}", x);
            break
        }
    }

    // 12, 35, 67?, 114, 136, 218, 237
    for i in (843+x*101)..(843+x*101)+1 {
        let lns: Vec<String> = lines.clone();
        let mut matrix: Vec<Vec<&str>> = vec![vec![" "; width.try_into().unwrap()]; height.try_into().unwrap()];
        for line in lns {
            let pv : Vec<&str> = line.split_whitespace().collect();
            let p: Vec<i32> = pv[0].split("=").collect::<Vec<&str>>()[1].split(",").filter_map(|s| s.parse::<i32>().ok()).collect();
            let v: Vec<i32> = pv[1].split("=").collect::<Vec<&str>>()[1].split(",").filter_map(|s| s.parse::<i32>().ok()).collect();
            // println!("Position: {:?}, Vector: {:?}", p, v);

            let after_iteration_x = (p[0] + v[0]*i).rem_euclid(width);
            let after_iteration_y = (p[1] + v[1]*i).rem_euclid(height);

            let new_x : usize = after_iteration_x.try_into().unwrap();
            let new_y : usize = after_iteration_y.try_into().unwrap();
            matrix[new_y][new_x] = "*";

            // if after_iteration_x < width/2 {
            //     if after_iteration_y < height/2 {
            //         q1c += 1;
            //     } else if after_iteration_y > height/2 {
            //         q2c += 1;
            //     }
            // } else if after_iteration_x > width/2{
            //     if after_iteration_y < height/2 {
            //         q3c += 1;
            //     } else if after_iteration_y > height/2 {
            //         q4c += 1;
            //     }
            // }
        }
        render(matrix, i);
    }
    Ok(()) 
    // println!("Q1: {}, Q2: {}, Q3: {}, Q4: {}", q1c, q2c, q3c, q4c);
    // println!("Mul: {}", q1c*q2c*q3c*q4c);
}


fn render(matrix: Vec<Vec<&str>>, i: i32) -> io::Result<()> {
    let filename = format!("output/output_{}.txt", i);
    let mut file = File::create(filename)?;

    for i in 0..matrix.len() {
        for &character in matrix[i].iter() {
            write!(file, "{}", character)?;
        }
        writeln!(file)?;
    }

    println!("Iteration: {}", i);
    Ok(())
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

