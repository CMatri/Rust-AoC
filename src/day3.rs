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

    for line in file_contents.lines() {
        let sub0: Vec<&str> = line.split("@").collect(); // { "#int", "int,int: intxint" }
        let sub1: Vec<&str> = sub0[1].split(":").collect(); // { "int,int", "intxint" }
        let filter = vec!['#', ' '];
        let id: String = sub0[0].chars().filter(|&c| !filter.contains(&c)).collect();
        let coords: String = sub1[0].chars().filter(|&c| !filter.contains(&c)).collect();
        let xy: Vec<&str> = coords.split(',').collect();
        let dimensions: String = sub1[1].chars().filter(|&c| !filter.contains(&c)).collect();
        let wh: Vec<&str> = dimensions.split('x').collect();
        rectangles.push(Rect { 
            id: id.parse().unwrap(), 
            x : xy[0].parse().unwrap(), 
            y : xy[1].parse().unwrap(),
            w : wh[0].parse().unwrap(),
            h : wh[1].parse().unwrap()
        });
    }

    for rect in rectangles {
        println!("id: {} xy: {}, {} wh: {}, {}", rect.id, rect.x, rect.y, rect.w, rect.h);
    }
}