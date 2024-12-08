use std::fs::read_to_string;
use std::collections::HashMap;


fn main() {
    let lines = read_lines("./input.txt");

    let mut nodes: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    for y in 0..lines.len() {
        let line = &lines[y];
        for x in 0..line.len() {
            if let Some(pos) = line.chars().nth(x) {
                if pos != '.' {
                    nodes.entry(pos).or_insert_with(Vec::new).push((x, y));
                }
            }
        }
    }

    let mut anti_nodes: HashMap<(i32, i32), i32> = HashMap::new();

    for (key, value) in &nodes {
        println!("{}: {:?}", key, value);
        for i in 0..value.len()-1 {
            for j in i+1..value.len() {
                let mut ix = value[i].0 as i32;
                let mut iy = value[i].1 as i32;
                let mut jx = value[j].0 as i32;
                let mut jy = value[j].1 as i32;
                anti_nodes.entry((ix, iy)).or_insert(0);
                anti_nodes.entry((jx, jy)).or_insert(0);

                let x_diff: i32 = ix-jx;
                let y_diff: i32 = iy-jy;

                if x_diff >= 0 && y_diff < 0 {
                    loop {
                        if ix+x_diff >= width || iy+y_diff < 0 {
                            break
                        }
                        ix += x_diff;
                        iy += y_diff;
                        anti_nodes.entry((ix, iy)).or_insert(0);
                    }

                    loop {
                        if jx-x_diff < 0 || jy-y_diff >= height {
                            break    
                        }
                        jx -= x_diff;
                        jy -= y_diff;
                        anti_nodes.entry((jx, jy)).or_insert(0);
                    }
                } else if x_diff < 0 && y_diff >= 0 {
                    loop {
                        if ix+x_diff < 0 || iy+y_diff >= height {
                            break
                        }
                        ix += x_diff;
                        iy += y_diff;
                        anti_nodes.entry((ix, iy)).or_insert(0);
                    }

                    loop {
                        if jx-x_diff >= width || jy-y_diff < 0 {
                            break    
                        }
                        jx -= x_diff;
                        jy -= y_diff;
                        anti_nodes.entry((jx, jy)).or_insert(0);
                    }
                } else {
                    loop {
                        if ix+x_diff < 0 || iy+y_diff < 0 {
                            break
                        }
                        ix += x_diff;
                        iy += y_diff;
                        anti_nodes.entry((ix, iy)).or_insert(0);
                    }

                    loop {
                        if jx-x_diff >= width || jy-y_diff >= height {
                            break    
                        }
                        jx -= x_diff;
                        jy -= y_diff;
                        anti_nodes.entry((jx, jy)).or_insert(0);
                    }
                }
            }
        }
    }
    println!("Anti nodes: {:?}", anti_nodes);

    println!("Count of anti nodes: {}", anti_nodes.len())
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
