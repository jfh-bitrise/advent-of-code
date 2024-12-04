use std::fs::read_to_string;

fn main() {
    let lines = read_lines("./input.txt");

    let mut xmases = 0;
    let mut crossmass = 0;
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let row: Vec<char> = line.chars().collect();
        data.push(row);
    }

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            xmases += count_xmas(data.clone(), j, i);
            crossmass += count_crossmas(data.clone(), j, i)
        }
    }

    println!("XMASes: {}", xmases);
    println!("Cross MASes: {}", crossmass)
}

fn count_crossmas(data: Vec<Vec<char>>, j: usize, i: usize) -> i32 {
    let mut crossmases = 0;

    if i <= data.len() - 3 && j <= data[i].len() - 3 {
        if data[i][j] == 'M' 
        && data[i+2][j] == 'M'
        && data[i+1][j+1] == 'A'
        && data[i][j+2] == 'S'
        && data[i+2][j+2] == 'S' {
            crossmases += 1;
        }

        if data[i][j] == 'M' 
        && data[i+2][j] == 'S'
        && data[i+1][j+1] == 'A'
        && data[i][j+2] == 'M'
        && data[i+2][j+2] == 'S' {
            crossmases += 1;
        }

        if data[i][j] == 'S' 
        && data[i+2][j] == 'M'
        && data[i+1][j+1] == 'A'
        && data[i][j+2] == 'S'
        && data[i+2][j+2] == 'M' {
            crossmases += 1;
        }

        if data[i][j] == 'S' 
        && data[i+2][j] == 'S'
        && data[i+1][j+1] == 'A'
        && data[i][j+2] == 'M'
        && data[i+2][j+2] == 'M' {
            crossmases += 1;
        }
    }
    
    return crossmases
}

fn count_xmas(data: Vec<Vec<char>>, j: usize, i: usize) -> i32 {
    let mut xmases = 0;
    // Vertical ltr
    if j <= data[i].len() - 4 {
        if data[i][j] == 'X' 
        && data[i][j+1] == 'M' 
        && data[i][j+2] == 'A'
        && data[i][j+3] == 'S' {
            xmases += 1
        }
    }
    // Vertical rtl
    if j >= 3 {
        if data[i][j] == 'X' 
        && data[i][j-1] == 'M' 
        && data[i][j-2] == 'A'
        && data[i][j-3] == 'S' {
            xmases += 1
        } 
    }
    // Horizontal utd
    if i <= data.len() - 4 {
        if data[i][j] == 'X' 
        && data[i+1][j] == 'M' 
        && data[i+2][j] == 'A'
        && data[i+3][j] == 'S' {
            xmases += 1
        } 
    }
    // Horizontal dtu
    if i >= 3 {
        if data[i][j] == 'X' 
        && data[i-1][j] == 'M' 
        && data[i-2][j] == 'A'
        && data[i-3][j] == 'S' {
            xmases += 1
        } 
    }
    // Downright
    if j <= data[i].len() - 4  && i <= data.len() - 4 {
        if data[i][j] == 'X' 
        && data[i+1][j+1] == 'M' 
        && data[i+2][j+2] == 'A'
        && data[i+3][j+3] == 'S' {
            xmases += 1
        }
    }
    // Upright
    if j <= data[i].len() - 4  && i >=3 {
        if data[i][j] == 'X' 
        && data[i-1][j+1] == 'M' 
        && data[i-2][j+2] == 'A'
        && data[i-3][j+3] == 'S' {
            xmases += 1
        }
    }
    // Downleft
    if j >= 3 && i <= data.len() - 4 {
        if data[i][j] == 'X' 
        && data[i+1][j-1] == 'M' 
        && data[i+2][j-2] == 'A'
        && data[i+3][j-3] == 'S' {
            xmases += 1
        }
    }
    // Upleft
    if j >= 3 && i >=3 {
        if data[i][j] == 'X' 
        && data[i-1][j-1] == 'M' 
        && data[i-2][j-2] == 'A'
        && data[i-3][j-3] == 'S' {
            xmases += 1
        }
    }
    return xmases
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

