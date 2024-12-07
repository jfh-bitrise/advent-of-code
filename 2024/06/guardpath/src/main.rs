use std::fs::read_to_string;

fn main() {
    let lines = read_lines("./input.txt");
    
    let mut pth_cnt = 0;
    let mut extra_obstacle_cnt = 0;
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let row: Vec<char> = line.chars().collect();
        data.push(row);
    }

    let mut starting_pos_x: i32 = 0;
    let mut starting_pos_y: i32 = 0;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == '^' {
                starting_pos_x = j as i32;
                starting_pos_y = i as i32;
            }
        }
    }
    let original_data = data.clone();
    
    let mut pos_x= starting_pos_x;
    let mut pos_y = starting_pos_y;
    let mut dir = "up".to_string();
    while pos_x >= 0 && pos_y >= 0 && pos_x <= (data[data.len()-1].len()-1) as i32 && pos_y <= (data.len()-1) as i32 {
        (data, pos_x, pos_y, dir, _) = move_and_mark(data, pos_x, pos_y, dir);
    }

    for i in 0..data.len() {
        // println!("{:?}", data[i]);
        for j in 0..data[i].len() {
            if data[i][j] == 'U' || data[i][j] == 'R' || data[i][j] == 'D' || data[i][j] == 'L' {
                pth_cnt += 1;
            }
        }
    }

    println!("Path count: {}", pth_cnt);
    

    for i in 0..original_data.len() {
        for j in 0..original_data[i].len() {
            // println!("Trial {} of {}", (i*original_data.len())+j+1, original_data.len()*original_data[0].len());
            if j as i32 == starting_pos_x && i as i32 == starting_pos_y {
                continue;
            }
            if original_data[i][j] != '#' 
                && (data[i][j] == 'U' || data[i][j] == 'R' || data[i][j] == 'D' || data[i][j] == 'L') {
                let mut trial_data = original_data.clone();
                trial_data[i][j] = '#';

                let mut pos_x: i32 = 0;
                let mut pos_y:i32 = 0;
                for i in 0..trial_data.len() {
                    for j in 0..trial_data[i].len() {
                        if trial_data[i][j] == '^' {
                            pos_x = j as i32;
                            pos_y = i as i32;
                        }
                    }
                }

                let mut updated_to_same = 0;
                let mut dir = "up".to_string();
                while pos_x >= 0 && pos_y >= 0 && pos_x <= (trial_data[trial_data.len()-1].len()-1) as i32 && pos_y <= (trial_data.len()-1) as i32 {
                    let a: i32;
                    (trial_data, pos_x, pos_y, dir, a) = move_and_mark(trial_data, pos_x, pos_y, dir);
                    updated_to_same += a;
                    if updated_to_same >= 10 {
                        extra_obstacle_cnt += 1;
                        break;
                    }
                }

            }
        }
    }
    
    
    
    println!("Xtra obstacle count: {}", extra_obstacle_cnt)
}

fn move_and_mark(data: Vec<Vec<char>>, pos_x: i32, pos_y: i32, dir: String) -> (Vec<Vec<char>>, i32, i32, String, i32) {
    let mut dd = data.clone();
    let p_y = pos_y as usize;
    let p_x = pos_x as usize;
    
    if dir == "up"{
        if p_y == 0 {
            dd[p_y][p_x] = 'U';
            return (dd, p_x as i32, p_y as i32 - 1, dir, 0)
        } else if dd[p_y-1][p_x] != '#' {
            if dd[p_y][p_x] == 'U'{
                return (dd, p_x as i32, p_y as i32 - 1, dir, 1)
            }
            dd[p_y][p_x] = 'U';
            return (dd, p_x as i32, p_y as i32 - 1, dir, 0)
        } else {
            return (dd, p_x as i32, p_y as i32, "right".to_string(), 0)
        }
    } else if dir == "right" {
        if p_x == dd[p_y].len() - 1 {
            dd[p_y][p_x] = 'R';
            return (dd, p_x as i32 + 1, p_y as i32, dir, 0)
        } else if dd[p_y][p_x+1] != '#' {
            if dd[p_y][p_x] == 'R'{
                return (dd, p_x as i32 + 1, p_y as i32, dir, 1)
            }
            dd[p_y][p_x] = 'R';
            return (dd, p_x as i32 + 1, p_y as i32, dir,0)
        } else {
            return (dd, p_x as i32, p_y as i32, "down".to_string(), 0)
        }
    } else if dir == "down" {
        if p_y == dd.len() - 1 {
            dd[p_y][p_x] = 'D';
            return (dd, p_x as i32, p_y as i32 + 1, dir, 0)
        } else if dd[p_y+1][p_x] != '#' {
            if dd[p_y][p_x] == 'D'{
                return (dd, p_x as i32, p_y as i32 + 1, dir, 1)
            }
            dd[p_y][p_x] = 'D';
            return (dd, p_x as i32, p_y as i32 + 1, dir, 0)
        } else {
            return (dd, p_x as i32, p_y as i32, "left".to_string(), 0)
        }
    } else if dir == "left" {
        if p_x == 0 {
            dd[p_y][p_x] = 'L';
            return (dd, p_x as i32 - 1, p_y as i32, dir, 0)
        } else if dd[p_y][p_x-1] != '#' {
            if dd[p_y][p_x] == 'L'{
                return (dd, p_x as i32 - 1, p_y as i32, dir, 1)
            }
            dd[p_y][p_x] = 'L';
            return (dd, p_x as i32 - 1, p_y as i32, dir, 0)
        } else {
            return (dd, p_x as i32, p_y as i32, "up".to_string(), 0)
        }
    }
    
    return (dd, pos_x, pos_y, dir, 0)
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
