use chrono::{DateTime, Month, Months, NaiveDate};
use std::io::Write;
use std::ops::Add;
use std::{env, error, io};
use std::{error::Error, fs, str::FromStr};

pub fn search(lifts: &Vec<Lift>, search_term: &str, sflag: char) -> Result<(), &'static str> {
    //
    match sflag {
        't' => {
            for lift in lifts {
                if lift.next_maintenance < NaiveDate::from_str(search_term).unwrap() {
                    println!("{}", lift.to_string());
                }
            }
        }
        'n' => {
            for lift in lifts {
                if lift.location == search_term || lift.name == search_term {
                    println!("{}", lift.to_string());
                }
            }
        }
        'a' => {
            for lift in lifts {
                println!("{}", lift.to_string());
            }
        }
        _ => return Err("No valid flag profided for searc"),
    }
    Ok(())
}

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

pub fn write_file(config: &Config, lifts: &Vec<Lift>) -> Result<(), &'static str> {
    //
    // fs::write(config.file_path, lifts);
    let mut data = String::new();
    for lift in lifts {
        data.push_str(&lift.to_string());
        data.push('\n');
    }
    fs::write(&config.file_path, &data).unwrap();
    println!("{}", data);
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

// add lift in database / lift.exe add <lift-id> -> location maintenance date
fn add_lift(arg: &String, lifts: &mut Vec<Lift>) -> Result<(), &'static str> {
    println!("Type a new lift instance in following pattern.");
    println!("location maintenance-date");
    loop {
        // get input to create the new lift
        let mut input = String::new();
        print!("input: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("error reading the line");
        let input: Vec<&str> = input.split_whitespace().collect();
        // check if there is enough items in the input
        if input.len() != 2 {
            println!("{:?}", input);
            println!("Not enough inputs");
            continue;
        }
        // assign the new naivedate to a variable and check for errors
        let maintenance = match NaiveDate::from_str(input[1]) {
            Ok(value) => value,
            Err(err) => {
                println!("Error: {err}");
                continue;
            }
        };
        // assign variable with current maintenance date + 3 months
        let next_maintenance = maintenance.checked_add_months(Months::new(3)).unwrap();
        let lift = Lift::build(
            arg.to_string(),
            input[0].to_string(),
            maintenance,
            next_maintenance,
        );
        lifts.push(lift);
        break;
    }
    io::stdout().flush().unwrap();
    Ok(())
}

fn parse_args(args: &Vec<String>, lifts: &mut Vec<Lift>) -> Result<(), &'static str> {
    if args.len() < 3 && args[1] != "list".to_string() {
        return Err("Too few arguments");
    }
    match args[1].as_str() {
        "add" => add_lift(&args[2], lifts).unwrap(),
        "list" => {
            if args.len() < 3 {
                search(&lifts, "all", 'a').unwrap();
            } else {
                search(&lifts, &args[2], 'n').unwrap();
            }
        }
        "delete" => println!("delete"),
        "fix" => println!("fix"),
        "edit" => println!("edit"),
        _ => return Err("Command not found"),
    }
    Ok(())
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
        println!("{}", self.to_string());
    }
    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {}",
            self.name, self.location, self.maintenance, self.next_maintenance
        )
        //
    }
}

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let config = Config::build("option".to_string(), "hissit.txt".to_string());
        let mut lifts = read_file(&config);
        parse_args(&args, &mut lifts).unwrap();
        for lift in &lifts {
            lift.print();
        }
        write_file(&config, &lifts).unwrap();
    } else {
        println!("Not enough arguments provided!");
    }
    // let lift = Lift::new();
    // lifts.push(lift);
    // write_file(&config, &lifts).unwrap();
}
