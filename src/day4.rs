use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct GuardEvent {
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    event: String,
    id: u32
}

fn parse_event(line: &str) -> GuardEvent {
    let year_rest: Vec<&str> = line.split("-").collect();
    let month = year_rest[1];
    let day_rest: Vec<&str> = year_rest[2].split(" ").collect();
    let day = day_rest[0];
    let event_rest: Vec<&str> = year_rest[2].split("]").collect();
    let event = event_rest[1];
    let time_stamp: Vec<&str> = day_rest[1].split(":").collect();
    let hour = time_stamp[0];
    let mut minute = time_stamp[1].to_string();
    minute.pop(); // remove ']'
    let id = format!("{}{}{}{}", month, day, hour, minute);
        
    GuardEvent {
        month: u32::from_str(month).unwrap(),
        day: u32::from_str(day).unwrap(),
        hour: u32::from_str(hour).unwrap(),
        minute: u32::from_str(&minute).unwrap(),
        event: event.to_string(),
        id: u32::from_str(&id).unwrap()
    }
}

fn main() {
    let filename = "day4.txt";
    let file_contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut events: Vec<GuardEvent> = Vec::new();

    for line in file_contents.lines() {
        events.push(parse_event(line));
    }

    events.sort_by_key(|e| e.id);
    for e in events {
        println!("{} {} {} {}", e.month, e.day, e.hour, e.minute);
    }
}
