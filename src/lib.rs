use chrono::{DateTime, NaiveDate};
use std::{error::Error, fs, str::FromStr};

pub fn read_file(config: &Config) -> Vec<Lift> {
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

pub fn write_file(config: &Config, lifts: &[Lift]) -> Result<(), &'static str> {
    //
    // fs::write(config.file_path, lifts);
    Ok(())
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
        chrono::NaiveDate::from_str(values[2]).unwrap(),
        chrono::NaiveDate::from_str(values[3]).unwrap(),
    )
}

pub struct Config {
    option: String,
    file_path: String,
}

impl Config {
    pub fn build(option: String, file_path: String) -> Config {
        Config {
            option: option,
            file_path: file_path,
        }
    }
}

#[derive(Debug)]
pub struct Lift {
    name: String,
    location: String,
    // maintenance: String,
    // next_maintenance: String,
    maintenance: chrono::NaiveDate,
    next_maintenance: chrono::NaiveDate,
}

impl Lift {
    fn new() -> Lift {
        Lift {
            name: "Test".to_string(),
            location: "Turku".to_string(),
            maintenance: chrono::NaiveDate::from_str("2023-1-1").unwrap(),
            next_maintenance: chrono::NaiveDate::from_str("2024-1-1").unwrap(),
            // maintenance: "1/1/2024".to_string(),
            // next_maintenance: "1/5/2025".to_string(),
        }
    }
    fn build(
        name: String,
        location: String,
        maintenance: chrono::NaiveDate,
        next_maintenance: chrono::NaiveDate,
    ) -> Lift {
        Lift {
            name: name,
            location: location,
            maintenance: maintenance,
            next_maintenance: next_maintenance,
        }
    }
    pub fn print(&self) {
        println!("{:?}", self);
    }
}

pub fn run() {
    let config = Config::build("option".to_string(), "hissit.txt".to_string());
    let mut lifts = read_file(&config);
    let lift = Lift::new();
    lifts.push(lift);
    for lift in lifts {
        lift.print();
    }
}
