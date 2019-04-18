use std::fs;

struct Rect {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32
}

fn main() {
    let filename = "day3.txt";
    let file_contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut rectangles: Vec<Rect> = Vec::new();
    let mut grid_width = 0;
    let mut grid_height = 0;

    for line in file_contents.lines() {
        let sub0: Vec<&str> = line.split("@").collect(); // { "#int", "int,int: intxint" }
        let sub1: Vec<&str> = sub0[1].split(":").collect(); // { "int,int", "intxint" }
        let filter = vec!['#', ' '];
        let id: String = sub0[0].chars().filter(|&c| !filter.contains(&c)).collect();
        let coords: String = sub1[0].chars().filter(|&c| !filter.contains(&c)).collect();
        let xy: Vec<&str> = coords.split(',').collect();
        let dimensions: String = sub1[1].chars().filter(|&c| !filter.contains(&c)).collect();
        let wh: Vec<&str> = dimensions.split('x').collect();
        let rect = Rect { 
            id: id.parse().unwrap(), 
            x : xy[0].parse().unwrap(), 
            y : xy[1].parse().unwrap(),
            w : wh[0].parse().unwrap(),
            h : wh[1].parse().unwrap()
        };

        if rect.x + rect.w > grid_width { grid_width = rect.x + rect.w; }
        if rect.y + rect.h > grid_height { grid_height = rect.y + rect.h; }

        rectangles.push(rect);
    }

    let mut grid = vec![vec![0u8; grid_height as usize]; grid_width as usize];

    for rect in &rectangles {
        for x in rect.x..rect.x+rect.w {
            for y in rect.y..rect.y+rect.h {
                grid[x as usize][y as usize] += 1;
            }
        }
    }

    let mut two_counts = 0;
    for line in &grid {
        two_counts += line.iter().filter(|&x| *x > 1).count();
    }
    println!("Count solution: {}", two_counts);


    for rect in &rectangles {
        let mut all_ones = true;
        for x in rect.x..rect.x+rect.w {
            for y in rect.y..rect.y+rect.h {
                if grid[x as usize][y as usize] != 1 { all_ones = false; }
            }
        }
        if all_ones { println!("ID solution: {}", rect.id); }
    }
}