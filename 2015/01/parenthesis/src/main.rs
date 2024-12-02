use std::fs;

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("sample_02.txt")
        .expect("Should have been able to read the file");
    let mut floor = 0;

    let cc = contents.as_bytes();
    let mut found_first_basement = false;

    for i in 0..cc.len(){
        if cc[i] == b'(' {
            floor += 1;
        } else if cc[i] == b')' {
            floor -= 1;
            if floor < 0 && !found_first_basement {
                found_first_basement = true;
                println!("The first occasion to enter the basement: {}", i+1)
            }
        }
    }
    println!("The expected floor of delivery is: {}", floor)
}
