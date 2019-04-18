use std::collections::HashSet;
use std::fs;

fn main() {
    let filename = "day1.txt";
    let file_contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let sum = file_contents.lines().map(|x| x.parse::<i32>().unwrap()).fold(0, |acc, x| acc + x);

    let parsed_contents = file_contents.lines().map(|x| x.parse::<i32>().unwrap()).cycle();
    let mut iter = 0;
    let mut already_found: HashSet<i32> = HashSet::new();
    
    let mut second_freq = 0;
    
    for p in parsed_contents {
        iter += p;
        if already_found.contains(&iter) {
            second_freq = iter;
            break;
        } else {
            already_found.insert(iter);
        }
    }

    println!("{}, {}", sum, second_freq);
}
