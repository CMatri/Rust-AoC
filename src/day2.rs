use std::fs;

fn main() {
    let filename = "day2.txt";
    let file_contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        
    let mut twos = 0;
    let mut threes = 0;
    let mut freq = [0u8; 256];
    
    for line in file_contents.lines() {
        for f in freq.iter_mut() { *f = 0u8; }
        for b in line.as_bytes() {
            freq[*b as usize] += 1;
        }
        if freq.iter().any(|&n| n == 2) { twos += 1; }
        if freq.iter().any(|&n| n == 3) { threes += 1; }
    }
    println!("Twos: {}, Threes: {}, Hash: {}", twos, threes, twos * threes);
    
    let mut differences;
    let mut dif_idx = 0;
    for line0 in file_contents.lines() {
        for line1 in file_contents.lines() {
            differences = 0;
            for (i, c) in line0.char_indices() {
                if c != line1.chars().nth(i).unwrap() { 
                    differences += 1;
                    dif_idx = i; 
                }
            }

            if differences == 1 { 
                let mut solution = String::from(line0);
                solution.remove(dif_idx);        
                println!("Found difference of one: {}, {}, Solution: {}", line0, line1, solution); 
                return;
            }
        }
    }
}
