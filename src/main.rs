// save things to a file
// parse lines from a file
// read things from a file
// hashmap
// dataformat
//      hissi sijainti {huoltoaika} {seuraava huolto}
// hissi1 porvoo 10.10.2023 10.4.2024
// the program should be able to read new lifts and able to tell
// when the next one should be fixed (4 months periods)

use chrono::DateTime;
use std::{env, fs, panic::Location};

fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config {
        file_path: "hissit.txt".to_string(),
        option: "Noption".to_string(),
    };
    let lifts = read_file(&config);
    for lift in lifts {
        lift.print();
    }
}

fn read_file(config: &Config) -> Vec<Lift> {
    // load in file in string
    let contents = fs::read_to_string(&config.file_path)
        .unwrap_or_else(|_| "Error reading the file contents".to_string());
    let mut lifts: Vec<Lift> = Vec::new();
    for line in contents.lines() {
        //
        lifts.push(parse_line(&line.to_string()));
    }
    return lifts;
}

fn parse_line(line: &String) -> Lift {
    // load in lines from a file
    // put the values from the line in Lift struct
    // put the struct in vector
    // return a Lift object
    let values: Vec<&str> = line.split_whitespace().collect();
    Lift::build(
        values[0].to_string(),
        values[1].to_string(),
        values[2].to_string(),
        values[3].to_string(),
    )
}

struct Config {
    option: String,
    file_path: String,
}

#[derive(Debug)]
struct Lift {
    name: String,
    location: String,
    maintenance: String,
    next_maintenance: String,
    // maintenance: chrono::NaiveDate,
    // next_maintenance: chrono::NaiveDate,
}

impl Lift {
    fn new() -> Lift {
        Lift {
            name: "Test".to_string(),
            location: "Turku".to_string(),
            maintenance: "1/1/2024".to_string(),
            next_maintenance: "1/5/2025".to_string(),
            // maintenance: "1/1/2024".to_string(),
            // next_maintenance: "1/5/2025".to_string(),
        }
    }
    fn build(
        name: String,
        location: String,
        maintenance: String,
        next_maintenance: String,
    ) -> Lift {
        Lift {
            name: name,
            location: location,
            maintenance: maintenance,
            next_maintenance: next_maintenance,
        }
    }
    fn print(&self) {
        println!("{:?}", self);
    }
}
